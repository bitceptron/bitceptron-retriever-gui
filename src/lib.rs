use std::sync::Arc;

use app_message::{setting_input_in_gui::SettingInputInGuiMessage, AppMessage};
use bitceptron_retriever::{
    client::BitcoincoreRpcClient,
    covered_descriptors::CoveredDescriptors,
    error::RetrieverError,
    explorer::Explorer,
    path_pairs::{PathDescriptorPair, PathScanResultDescriptorTrio},
    retriever::Retriever,
    setting::RetrieverSetting,
    uspk_set::UnspentScriptPupKeysSet,
};
use iced::{
    executor,
    widget::{text_editor, Column},
    Application, Command,
};
use inputs::{
    bitcoincore_client::BitcoincoreClientInput, explorer::ExplorerInput,
    retriever_specific::RetrieverSpecificInput,
};
use run_functions::{
    check_for_dump_in_data_dir_or_create_dump_file, create_final_finds, create_retriever_setting,
    get_details_of_finds_from_bitcoincore, populate_uspk_set, search_the_uspk_set,
};
use tracing_log::log::info;
use view_elements::{
    bitcoincore_client_setting_row, exploration_setting_row, results_row::results_row,
    retriever_setting_row, run_row::run_row,
};

pub mod app_message;
pub mod app_status;
pub mod domain;
pub mod gui_error;
pub mod inputs;
pub mod retriever_styles;
pub mod run_functions;
pub mod status;
pub mod view_elements;

#[derive(Debug)]
pub struct RetrieverApp {
    bitcoincore_client_setting_input: BitcoincoreClientInput,
    explorer_setting_input: ExplorerInput,
    retriever_specific_setting_input: RetrieverSpecificInput,
    mnemonic_content: text_editor::Content,
    retriever_setting: Option<RetrieverSetting>,
    client: BitcoincoreRpcClient,
    explorer: Arc<Explorer>,
    uspk_set: Arc<UnspentScriptPupKeysSet>,
    data_dir: String,
    finds: Vec<PathDescriptorPair>,
    detailed_finds: Option<Vec<PathScanResultDescriptorTrio>>,
    final_finds: Vec<String>,
    select_descriptors: hashbrown::HashSet<CoveredDescriptors>,
    errors: Vec<Arc<RetrieverError>>,
    is_retriever_built: bool,
    is_dump_file_ready: bool,
    is_utxo_set_ready: bool,
}

impl Default for RetrieverApp {
    fn default() -> Self {
        Self {
            bitcoincore_client_setting_input: Default::default(),
            explorer_setting_input: Default::default(),
            retriever_specific_setting_input: Default::default(),
            mnemonic_content: Default::default(),
            retriever_setting: None,
            errors: vec![],
            is_retriever_built: false,
            is_dump_file_ready: false,
            is_utxo_set_ready: false,
            client: Default::default(),
            explorer: Default::default(),
            uspk_set: Default::default(),
            data_dir: Default::default(),
            finds: Default::default(),
            detailed_finds: Default::default(),
            final_finds: Default::default(),
            select_descriptors: Default::default(),
        }
    }
}

impl Application for RetrieverApp {
    type Executor = executor::Default;

    type Message = AppMessage;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let app = RetrieverApp::default();
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("bitceptron retriever")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            AppMessage::SettingInputInGuiChanged(input_change) => match input_change {
                SettingInputInGuiMessage::BitcoincoreUrlChanged(new_url) => {
                    self.bitcoincore_client_setting_input.set_url_from_gui_input(new_url)
                }
                SettingInputInGuiMessage::BitcoincoreRpcPortChanged(new_rpc_port) => self
                    .bitcoincore_client_setting_input
                    .set_rpc_port_from_gui_input(new_rpc_port),
                SettingInputInGuiMessage::BitcoincoreTimeoutChanged(new_timeout) => self
                    .bitcoincore_client_setting_input
                    .set_timeout_from_gui_input(new_timeout),
                SettingInputInGuiMessage::BitcoincoreCookiePathChanged(new_cookie_path) => self
                    .bitcoincore_client_setting_input
                    .set_cookie_path_from_gui_input(new_cookie_path),
                SettingInputInGuiMessage::NetworkChanged(network) => self.explorer_setting_input.set_network_from_gui_input(network),
                SettingInputInGuiMessage::MnemonicChanged(action) => {
                    self.explorer_setting_input.update_mnemonic_from_gui_input(action.clone());
                    self.mnemonic_content.perform(action)
                },
                SettingInputInGuiMessage::BaseDerivationPathsChanged(base_derivation_paths) => self.explorer_setting_input.set_base_derivation_paths_from_gui_input(base_derivation_paths),
                SettingInputInGuiMessage::BaseDerivationPathsFromPresetsChanged(base_derivation_paths_from_presets) => {
                    self.explorer_setting_input.set_base_derivation_paths_from_presets_from_gui_input(base_derivation_paths_from_presets);
                    if base_derivation_paths_from_presets {
                        self.explorer_setting_input.set_base_derivation_paths_from_gui_input(bitceptron_retriever::data::wallets_info::WalletsInfo::get_all_unique_preset_wallet_base_paths().join(";"))
                    }
                },
                SettingInputInGuiMessage::ExplorationPathChanged(exploration_path) => self.explorer_setting_input.set_exploration_path_from_gui_input(exploration_path),
                SettingInputInGuiMessage::SweepChanged(sweep) => self.explorer_setting_input.set_sweep_from_gui_input(sweep),
                SettingInputInGuiMessage::ExplorationDepthChanged(exploration_dept) => self.explorer_setting_input.set_exploration_depth_from_gui_input(exploration_dept),
                SettingInputInGuiMessage::P2pkInclusionChanged(p2pk_inclusion) => self.retriever_specific_setting_input.set_p2pk_inclusion_from_gui_input(p2pk_inclusion),
                SettingInputInGuiMessage::P2pkhInclusionChanged(p2pkh_inclusion) => self.retriever_specific_setting_input.set_p2pkh_inclusion_from_gui_input(p2pkh_inclusion),
                SettingInputInGuiMessage::P2wpkhInclusionChanged(p2wpkh_inclusion) => self.retriever_specific_setting_input.set_p2wpkh_inclusion_from_gui_input(p2wpkh_inclusion),
                SettingInputInGuiMessage::P2shwpkhInclusionChanged(p2shwpkh_inclusion) => self.retriever_specific_setting_input.set_p2shwpkh_inclusion_from_gui_input(p2shwpkh_inclusion),
                SettingInputInGuiMessage::P2trInclusionChanged(p2tr_inclusion) => self.retriever_specific_setting_input.set_p2tr_inclusion_from_gui_input(p2tr_inclusion),
                SettingInputInGuiMessage::DataDirChanged(data_dir) => self.retriever_specific_setting_input.set_data_dir_from_gui_input(data_dir),
                SettingInputInGuiMessage::PassphraseChanged(passphrase) => self.explorer_setting_input.set_passphrase_from_gui_input(passphrase),
            },
            AppMessage::SettingInputGotFixed(input_fixed) => match input_fixed {
                app_message::setting_input_fixed::SettingInputFixedMessage::BitcoincoreClientSettingFixed => {
                    let _ = self.bitcoincore_client_setting_input.gui_to_in_use();
                },
                app_message::setting_input_fixed::SettingInputFixedMessage::ExplorerSettingFixed => {
                    let _ = self.explorer_setting_input.gui_to_in_use();
                },
                app_message::setting_input_fixed::SettingInputFixedMessage::RetrieverSettingFixed => {let _ = self.retriever_specific_setting_input.gui_to_in_use();},
            },
            AppMessage::CreateRetriever => {
                self.retriever_setting = create_retriever_setting(self);
                let setting = Arc::new(self.retriever_setting.as_ref().unwrap().clone());
                return Command::perform(Retriever::new(setting), |result| match result {
                    Ok(retriever) => AppMessage::RetrieverCreated(retriever),
                    Err(error) => AppMessage::Error(Arc::new(error)),
                });
            },
            AppMessage::PrepareDumpFile => {
                let data_dir = self.data_dir.clone();
                let client = self.client.clone();
                return Command::perform(check_for_dump_in_data_dir_or_create_dump_file(data_dir, client), |dump_result| {
                    match dump_result {
                        Ok(_) => AppMessage::DumpFileDone,
                        Err(e) => AppMessage::Error(Arc::new(e)),
                    }
                });
            },
            AppMessage::Search => {
                let select_descriptors = self.select_descriptors.clone();
                let uspk_set = self.uspk_set.clone();
                let explorer = self.explorer.clone();
                return Command::perform(search_the_uspk_set(
                    select_descriptors,
                    uspk_set,
                    explorer,
                    ), |search_result| match search_result {
                        Ok(result) => AppMessage::SearchResultPrepared(result),
                        Err(e) => AppMessage::Error(Arc::new(e)),
                    });
            },
            AppMessage::RetrieverCreated(retriever) => {
                self.client = retriever.client().clone();
                self.explorer = retriever.explorer().clone();
                self.uspk_set = Arc::new(retriever.uspk_set().clone());
                self.data_dir = retriever.data_dir().clone();
                self.finds = retriever.finds().lock().unwrap().clone();
                self.detailed_finds = retriever.detailed_finds().clone();
                self.select_descriptors = retriever.select_descriptors().clone();
                self.is_retriever_built = true;
                info!("{:?}", self.explorer);
                info!("{:?}", self.explorer_setting_input.get_in_use_network());
            },
            AppMessage::Error(e) => self.errors.push(e),
            AppMessage::None => {},
            AppMessage::DumpFileDone => self.is_dump_file_ready = true,
            AppMessage::PopulateUtxoDB => {
                let data_dir = self.data_dir.clone();
                return Command::perform(populate_uspk_set(data_dir), |populate_result| match populate_result {
                    Ok(set) => AppMessage::SetPopulated(set),
                    Err(e) => AppMessage::Error(Arc::new(e)),
                });
            },
            AppMessage::SetPopulated(set) => {
                self.uspk_set = Arc::new(set);
                self.is_utxo_set_ready = true},
            AppMessage::SearchResultPrepared(search_result) => {
                self.finds = search_result;
            },
            AppMessage::GetDetails => {
                let finds = self.finds.clone();
                let client = self.client.clone();
                return Command::perform(get_details_of_finds_from_bitcoincore(finds, client), |details_result| match details_result {
                    Ok(details) => AppMessage::DetailsReady(details),
                    Err(e) => AppMessage::Error(Arc::new(e)),
                });
            },
            AppMessage::DetailsReady(details) => {
                self.detailed_finds = details.clone();
                match create_final_finds(details) {
                    Ok(final_finds) => {
                        self.final_finds = final_finds; 
                    },
                Err(e) => self.errors.push(Arc::new(e)),
                }
            },
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        Column::new()
            .push(bitcoincore_client_setting_row(self))
            .push(exploration_setting_row(self))
            .push(retriever_setting_row(self))
            .push(run_row(self))
            .push(results_row(self))
            .into()
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::TokyoNight
    }
}
