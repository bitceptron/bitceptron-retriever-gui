use std::{
    error::Error,
    fs,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

use bitceptron_retriever::{
    client::{dump_utxout_set_result::DumpTxoutSetResult, BitcoincoreRpcClient},
    covered_descriptors::CoveredDescriptors,
    error::RetrieverError,
    explorer::Explorer,
    path_pairs::{PathDescriptorPair, PathScanResultDescriptorTrio},
    setting::RetrieverSetting,
    uspk_set::{UnspentScriptPupKeysSet, UspkSetStatus},
};
use bitcoin::{bip32::DerivationPath, key::Secp256k1};
use itertools::Itertools;
use miniscript::Descriptor;
use num_format::{Locale, ToFormattedString};
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use crate::RetrieverApp;

pub fn create_retriever_setting(app: &mut RetrieverApp) -> Option<RetrieverSetting> {
    if app.bitcoincore_client_setting_input.is_input_fixed()
        && app.explorer_setting_input.is_input_fixed()
        && app.retriever_specific_setting_input.is_input_fixed()
    {
        Some(RetrieverSetting::new(
            Some(app.bitcoincore_client_setting_input.get_in_use_url()),
            Some(app.bitcoincore_client_setting_input.get_in_use_rpc_port()),
            app.bitcoincore_client_setting_input
                .get_in_use_cookie_path(),
            Some(app.bitcoincore_client_setting_input.get_in_use_timeout()),
            app.explorer_setting_input.get_in_use_mnemonic(),
            app.explorer_setting_input.get_in_use_passphrase(),
            Some(
                app.explorer_setting_input
                    .get_in_use_base_derivation_paths(),
            ),
            Some(app.explorer_setting_input.get_in_use_exploration_path()),
            Some(
                app.retriever_specific_setting_input
                    .get_in_use_selected_descriptors(),
            ),
            Some(app.explorer_setting_input.get_in_use_sweep()),
            Some(app.explorer_setting_input.get_in_use_exploration_depth()),
            Some(app.explorer_setting_input.get_in_use_network()),
            app.retriever_specific_setting_input.get_in_use_data_dir(),
        ))
    } else {
        None
    }
}

pub async fn check_for_dump_in_data_dir_or_create_dump_file(
    data_dir: String,
    client: BitcoincoreRpcClient,
) -> Result<(), RetrieverError> {
    let res = tokio::join!(async {
        let data_dir_path = PathBuf::from_str(&data_dir).unwrap();
        let mut dump_file_path = data_dir_path.clone();
        dump_file_path.extend(["utxo_dump.dat"]);
        info!("Searching for the dump file in datadir.");
        if dump_file_path.exists() {
            info!("Dump file found in datadir.");
            Ok(())
        } else {
            info!("Dump file was not found in datadir.");
            if !data_dir_path.exists() {
                info!("Creating the full datadir path.");
                fs::create_dir_all(data_dir_path)?;
            }
            let _dump_result = client.dump_utxo_set(&data_dir).await?;
            Ok(())
        }
    });
    res.0
}

pub async fn populate_uspk_set(
    data_dir: String,
) -> Result<UnspentScriptPupKeysSet, RetrieverError> {
    let mut uspk_set = UnspentScriptPupKeysSet::new();
    if uspk_set.get_status() == UspkSetStatus::Empty {
        info!("Searching for the dump file to populate the Unspent ScriptPubKey set.");
        let dump_file_path_str = format!("{}/utxo_dump.dat", data_dir);
        let dump_file_path = PathBuf::from_str(&dump_file_path_str).unwrap();
        if !dump_file_path.exists() {
            error!("Dump file (utxo_dump.dat) does not exist in data dir.");
            return Err(RetrieverError::NoDumpFileInDataDir);
        }
        info!("Dump file found.");
        let _ = tokio::join!({ uspk_set.populate_with_dump_file(&dump_file_path_str) });
        Ok(uspk_set)
    } else if uspk_set.get_status() == UspkSetStatus::Populating {
        Err(RetrieverError::PopulatingUSPKSetInProgress)
    } else {
        Err(RetrieverError::USPKSetAlreadyPopulated)
    }
}

pub async fn create_derivation_path_stream(
    explorer: Arc<Explorer>,
    sender: mpsc::Sender<DerivationPath>,
) -> Result<(), RetrieverError> {
    let bases = explorer.get_exploration_path().get_base_paths().to_owned();
    let num_explore_paths = explorer.get_exploration_path().size();
    let total_paths = num_explore_paths * bases.len();
    let mut sent_paths = 0;
    tokio::spawn(async move {
        info!(
            "Creation of an iterator for total {} paths started.",
            total_paths.to_formatted_string(&Locale::en)
        );
        let explore_paths_iter = explorer
            .get_exploration_path()
            .clone()
            .get_explore()
            .to_owned()
            .iter()
            .map(|step| step.to_owned())
            .multi_cartesian_product();
        for explore_path in explore_paths_iter {
            for base in bases.iter() {
                sender
                    .send(base.extend(
                        DerivationPath::from_str(&format!("m/{}", explore_path.join("/"))).unwrap(),
                    ))
                    .await
                    .unwrap();
                sent_paths += 1;
                if sent_paths % 1000 == 0 {
                    info!(
                        "Total paths sent to processing: {} of {}",
                        sent_paths.to_formatted_string(&Locale::en),
                        total_paths.to_formatted_string(&Locale::en)
                    )
                }
            }
        }
    });
    Ok(())
}

pub async fn process_derivation_path_stream(
    select_descriptors: hashbrown::HashSet<CoveredDescriptors>,
    uspk_set: Arc<UnspentScriptPupKeysSet>,
    explorer: Arc<Explorer>,
    receiver: &mut mpsc::Receiver<DerivationPath>,
) -> Result<Vec<PathDescriptorPair>, RetrieverError> {
    let secp = Secp256k1::new();
    let uspk_set = uspk_set.get_immutable_inner_set();
    let mut paths_received = 0;
    let mut finds = vec![];
    while let Some(path) = receiver.recv().await {
        paths_received += 1;
        if paths_received % 1000 == 0 {
            info!(
                "Total paths received to process: {}",
                paths_received.to_formatted_string(&Locale::en)
            );
        }
        let pubkey = explorer
            .get_master_xpriv()
            .derive_priv(&secp, &path)
            .unwrap()
            .to_keypair(&secp)
            .public_key();
        if select_descriptors.contains(&CoveredDescriptors::P2pk) {
            let desc = Descriptor::new_pk(pubkey);
            let desc_pubkey = desc.script_pubkey();
            let target = desc_pubkey.as_bytes();
            if uspk_set.contains(target) {
                warn!("Found a UTXO match for ScriptPubKey.");
                finds.push(PathDescriptorPair::new(path.to_owned(), desc));
            }
        }
        if select_descriptors.contains(&CoveredDescriptors::P2pkh) {
            let desc = Descriptor::new_pkh(pubkey)
                .map_err(RetrieverError::from)
                .unwrap();
            let desc_pubkey = desc.script_pubkey();
            let target = desc_pubkey.as_bytes();
            if uspk_set.contains(target) {
                warn!("Found a UTXO match for ScriptPubKey.");
                finds.push(PathDescriptorPair::new(path.to_owned(), desc));
            }
        }
        if select_descriptors.contains(&CoveredDescriptors::P2wpkh) {
            let desc = Descriptor::new_wpkh(pubkey)
                .map_err(RetrieverError::from)
                .unwrap();
            let desc_pubkey = desc.script_pubkey();
            let target = desc_pubkey.as_bytes();
            if uspk_set.contains(target) {
                warn!("Found a UTXO match for ScriptPubKey.");
                finds.push(PathDescriptorPair::new(path.to_owned(), desc));
            }
        }
        if select_descriptors.contains(&CoveredDescriptors::P2shwpkh) {
            let desc = Descriptor::new_sh_wpkh(pubkey)
                .map_err(RetrieverError::from)
                .unwrap();
            let desc_pubkey = desc.script_pubkey();
            let target = desc_pubkey.as_bytes();
            if uspk_set.contains(target) {
                warn!("Found a UTXO match for ScriptPubKey.");
                finds.push(PathDescriptorPair::new(path.to_owned(), desc));
            }
        }
        if select_descriptors.contains(&CoveredDescriptors::P2tr) {
            let desc = Descriptor::new_tr(pubkey, None)
                .map_err(RetrieverError::from)
                .unwrap();
            let desc_pubkey = desc.script_pubkey();
            let target = desc_pubkey.as_bytes();
            if uspk_set.contains(target) {
                warn!("Found a UTXO match for ScriptPubKey.");
                finds.push(PathDescriptorPair::new(path.to_owned(), desc));
            }
        }
    }
    Ok(finds)
}

pub async fn search_the_uspk_set(
    select_descriptors: hashbrown::HashSet<CoveredDescriptors>,
    uspk_set: Arc<UnspentScriptPupKeysSet>,
    explorer: Arc<Explorer>,
) -> Result<Vec<PathDescriptorPair>, RetrieverError> {
    let (tx, mut rx) = mpsc::channel(1024);
    let _ = tokio::join!(create_derivation_path_stream(explorer.clone(), tx));
    let res = tokio::join!(process_derivation_path_stream(
        select_descriptors,
        uspk_set,
        explorer,
        &mut rx
    ));
    res.0
}

pub async fn get_details_of_finds_from_bitcoincore(
    finds: Vec<PathDescriptorPair>,
    client: BitcoincoreRpcClient,
) -> Result<Option<Vec<PathScanResultDescriptorTrio>>, RetrieverError> {
    if finds.is_empty() {
        println!("No UTXO match were found in the explored paths.");
        Ok(None)
    } else {
        let path_scan_request_pairs = finds
            .iter()
            .map(|item| item.to_path_scan_request_descriptor_trio())
            .collect();
        let detailed_finds = Some(client.scan_utxo_set(path_scan_request_pairs).await?);
        Ok(detailed_finds)
    }
}


pub fn create_final_finds(detailed_finds: Option<Vec<PathScanResultDescriptorTrio>>) -> Result<Vec<String>, RetrieverError> {
        if detailed_finds.is_none() {
            return Err(RetrieverError::DetailsHaveNotBeenFetched);
        };
        let mut res = vec![];
        for (index, detail) in detailed_finds.unwrap().iter().enumerate() {
            let info = format!(
                "Result {}\nPath: {}\nAmount(satoshis): {}\nDescriptor: {}",
                index + 1,
                detail.0,
                detail
                    .1
                    .total_amount
                    .to_sat()
                    .to_formatted_string(&Locale::en),
                detail.2
            );
            res.push( info);
        }
        Ok(res)
    }
