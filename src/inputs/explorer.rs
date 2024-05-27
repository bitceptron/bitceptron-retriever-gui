use std::{path::PathBuf, str::FromStr};

use bitceptron_retriever::{
    covered_descriptors::CoveredDescriptors,
    data::defaults::{DEFAULT_EXPLORATION_DEPTH, DEFAULT_EXPLORATION_PATH},
    explorer::exploration_path::ExplorationPath,
};
use bitcoin::bip32::DerivationPath;
use iced::widget::text_editor;

use crate::gui_error::GuiError;

use super::gui_input::GuiInput;

#[derive(Debug)]
pub struct ExplorerInput {
    gui_input: ExplorerSettingFromGui,
    in_use: Option<ExplorerSettingInUse>,
}

impl Default for ExplorerInput {
    fn default() -> Self {
        Self {
            gui_input: Default::default(),
            in_use: None,
        }
    }
}

impl ExplorerInput {
    pub fn new() -> Self {
        ExplorerInput::default()
    }

    pub fn gui_to_in_use(&mut self) -> Result<(), GuiError> {
        let in_use = match self.is_gui_input_sane() {
            true => ExplorerSettingInUse {
                in_use_base_derivation_paths: self
                    .get_gui_base_derivation_paths()
                    .split(";")
                    .map(|path| path.to_string())
                    .collect(),
                in_use_base_derivation_paths_from_presets: self
                    .get_gui_base_derivation_paths_from_presets(),
                in_use_exploration_path: self.get_in_use_exploration_path(),
                in_use_sweep: self.get_in_use_sweep(),
                in_use_exploration_depth: self.get_gui_exploration_depth().parse::<u32>().unwrap(),
                in_use_network: self.get_gui_network(),
                in_use_selected_descriptors: {
                    let mut res = vec![];
                    if self.get_gui_p2pk() {
                        res.push(CoveredDescriptors::P2pk)
                    };
                    if self.get_gui_p2pkh() {
                        res.push(CoveredDescriptors::P2pkh)
                    };

                    if self.get_gui_p2wpkh() {
                        res.push(CoveredDescriptors::P2wpkh)
                    };

                    if self.get_gui_p2shwpkh() {
                        res.push(CoveredDescriptors::P2shwpkh)
                    };

                    if self.get_gui_p2tr() {
                        res.push(CoveredDescriptors::P2tr)
                    };
                    res
                },
                in_use_data_dir: self.get_gui_data_dir(),
                in_use_mnemonic: self.get_gui_mnemonic(),
                in_use_passphrase: self.get_gui_passphrase(),
            },
            false => return Err(GuiError::GuiInputIsInsane),
        };
        self.in_use = Some(in_use);
        Ok(())
    }

    pub fn set_base_derivation_paths_from_gui_input(&mut self, base_derivation_paths: String) {
        self.gui_input.gui_base_derivation_paths =
            BaseDerivationPathsGuiData::new(base_derivation_paths)
    }

    pub fn set_gui_base_derivation_paths_from_presets_from_gui_input(
        &mut self,
        base_derivation_paths_from_presets: bool,
    ) {
        self.gui_input.gui_base_derivation_paths_from_presets = base_derivation_paths_from_presets
    }

    pub fn set_exploration_path_from_gui_input(&mut self, exploration_path: String) {
        self.gui_input.gui_exploration_path = ExplorationPathGuiData::new(exploration_path)
    }

    pub fn set_sweep_from_gui_input(&mut self, sweep: bool) {
        self.gui_input.gui_sweep = sweep
    }

    pub fn set_exploration_depth_from_gui_input(&mut self, exploration_dept: String) {
        self.gui_input.gui_exploration_depth = ExplorationDepthGuiData::new(exploration_dept)
    }

    pub fn set_network_from_gui_input(&mut self, network: bitcoin::Network) {
        self.gui_input.gui_network = NetworkGuiData::new(network)
    }

    pub fn set_p2pk_inclusion_from_gui_input(&mut self, p2pk_inclusion: bool) {
        self.gui_input.gui_p2pk = p2pk_inclusion
    }

    pub fn set_p2pkh_inclusion_from_gui_input(&mut self, p2pkh_inclusion: bool) {
        self.gui_input.gui_p2pkh = p2pkh_inclusion
    }

    pub fn set_p2wpkh_inclusion_from_gui_input(&mut self, p2wpkh_inclusion: bool) {
        self.gui_input.gui_p2wpkh = p2wpkh_inclusion
    }

    pub fn set_p2shwpkh_inclusion_from_gui_input(&mut self, p2shwpkh_inclusion: bool) {
        self.gui_input.gui_p2shwpkh = p2shwpkh_inclusion
    }

    pub fn set_p2tr_inclusion_from_gui_input(&mut self, p2tr_inclusion: bool) {
        self.gui_input.gui_p2tr = p2tr_inclusion
    }

    pub fn set_data_dir_from_gui_input(&mut self, data_dir: String) {
        self.gui_input.gui_data_dir = DataDirGuiData::new(data_dir)
    }

    pub fn update_mnemonic_from_gui_input(&mut self, action: text_editor::Action) {
        self.gui_input.gui_mnemonic.update(action)
    }

    pub fn set_passphrase_from_gui_input(&mut self, passphrase: String) {
        self.gui_input.gui_passphrase = PassphraseGuiData::new(passphrase)
    }

    pub fn get_gui_base_derivation_paths(&self) -> String {
        self.gui_input.gui_base_derivation_paths.get_value()
    }

    pub fn get_gui_base_derivation_paths_from_presets(&self) -> bool {
        self.gui_input.gui_base_derivation_paths_from_presets
    }

    pub fn get_gui_exploration_path(&self) -> String {
        self.gui_input.gui_exploration_path.get_value()
    }

    pub fn get_gui_sweep(&self) -> bool {
        self.gui_input.gui_sweep
    }

    pub fn get_gui_exploration_depth(&self) -> String {
        self.gui_input.gui_exploration_depth.get_value()
    }

    pub fn get_gui_network(&self) -> bitcoin::Network {
        self.gui_input.gui_network.get_value()
    }

    pub fn get_gui_p2pk(&self) -> bool {
        self.gui_input.gui_p2pk
    }

    pub fn get_gui_p2pkh(&self) -> bool {
        self.gui_input.gui_p2pkh
    }

    pub fn get_gui_p2wpkh(&self) -> bool {
        self.gui_input.gui_p2wpkh
    }

    pub fn get_gui_p2shwpkh(&self) -> bool {
        self.gui_input.gui_p2shwpkh
    }

    pub fn get_gui_p2tr(&self) -> bool {
        self.gui_input.gui_p2tr
    }

    pub fn get_gui_data_dir(&self) -> String {
        self.gui_input.gui_data_dir.get_value()
    }

    pub fn get_gui_mnemonic(&self) -> String {
        self.gui_input.gui_mnemonic.get_value()
    }

    pub fn get_gui_passphrase(&self) -> String {
        self.gui_input.gui_passphrase.get_value()
    }

    pub fn get_in_use_base_derivation_paths(&self) -> String {
        match &self.in_use {
            Some(in_use) => in_use.in_use_base_derivation_paths.join(";"),
            None => "".to_string(),
        }
    }

    pub fn get_in_use_base_derivation_paths_from_presets(&self) -> bool {
        match &self.in_use {
            Some(in_use) => in_use.in_use_base_derivation_paths_from_presets,
            None => false,
        }
    }

    pub fn get_in_use_exploration_path(&self) -> String {
        match &self.in_use {
            Some(in_use) => in_use.in_use_exploration_path.to_owned(),
            None => "".to_string(),
        }
    }

    pub fn get_in_use_sweep(&self) -> bool {
        match &self.in_use {
            Some(in_use) => in_use.in_use_sweep,
            None => false,
        }
    }

    pub fn get_in_use_exploration_depth(&self) -> String {
        match &self.in_use {
            Some(in_use) => in_use.in_use_exploration_depth.to_string(),
            None => "".to_string(),
        }
    }

    pub fn get_in_use_network(&self) -> bitcoin::Network {
        match &self.in_use {
            Some(in_use) => in_use.in_use_network,
            None => bitcoin::Network::Bitcoin,
        }
    }

    pub fn get_in_use_p2pk(&self) -> bool {
        match &self.in_use {
            Some(in_use) => in_use
                .in_use_selected_descriptors
                .contains(&CoveredDescriptors::P2pk),
            None => false,
        }
    }

    pub fn get_in_use_p2pkh(&self) -> bool {
        match &self.in_use {
            Some(in_use) => in_use
                .in_use_selected_descriptors
                .contains(&CoveredDescriptors::P2pkh),
            None => false,
        }
    }

    pub fn get_in_use_p2wpkh(&self) -> bool {
        match &self.in_use {
            Some(in_use) => in_use
                .in_use_selected_descriptors
                .contains(&CoveredDescriptors::P2wpkh),
            None => false,
        }
    }

    pub fn get_in_use_p2shwpkh(&self) -> bool {
        match &self.in_use {
            Some(in_use) => in_use
                .in_use_selected_descriptors
                .contains(&CoveredDescriptors::P2shwpkh),
            None => false,
        }
    }

    pub fn get_in_use_p2tr(&self) -> bool {
        match &self.in_use {
            Some(in_use) => in_use
                .in_use_selected_descriptors
                .contains(&CoveredDescriptors::P2tr),
            None => false,
        }
    }

    pub fn get_in_use_data_dir(&self) -> String {
        match &self.in_use {
            Some(in_use) => in_use.in_use_data_dir.to_string(),
            None => "".to_string(),
        }
    }

    pub fn get_in_use_mnemonic(&self) -> String {
        match &self.in_use {
            Some(in_use) => in_use.in_use_mnemonic.to_string(),
            None => "".to_string(),
        }
    }

    pub fn get_in_use_passphrase(&self) -> String {
        match &self.in_use {
            Some(in_use) => in_use.in_use_passphrase.to_string(),
            None => "".to_string(),
        }
    }

    pub fn is_gui_base_derivation_paths_sane(&self) -> bool {
        self.gui_input.gui_base_derivation_paths.is_sane()
    }

    pub fn is_gui_exploration_path_sane(&self) -> bool {
        self.gui_input.gui_exploration_path.is_sane()
    }

    pub fn is_gui_exploration_depth_sane(&self) -> bool {
        self.gui_input.gui_exploration_depth.is_sane()
    }

    pub fn is_gui_selected_descriptors_sane(&self) -> bool {
        self.get_gui_p2pk()
            || self.get_gui_p2pkh()
            || self.get_gui_p2wpkh()
            || self.get_gui_p2shwpkh()
            || self.get_gui_p2tr()
    }

    pub fn is_gui_data_dir_sane(&self) -> bool {
        self.gui_input.gui_data_dir.is_sane()
    }

    pub fn is_gui_mnemonic_sane(&self) -> bool {
        self.gui_input.gui_mnemonic.is_sane()
    }

    pub fn is_gui_passphrase_sane(&self) -> bool {
        self.gui_input.gui_passphrase.is_sane()
    }

    pub fn is_gui_input_sane(&self) -> bool {
        self.is_gui_base_derivation_paths_sane()
            && self.is_gui_data_dir_sane()
            && self.is_gui_exploration_depth_sane()
            && self.is_gui_exploration_path_sane()
            && self.is_gui_passphrase_sane()
            && self.is_gui_selected_descriptors_sane()
            && self.is_gui_mnemonic_sane()
    }

    pub fn is_base_derivation_paths_fixed(&self) -> bool {
        self.in_use.is_some()
            && (self.get_gui_base_derivation_paths() == self.get_in_use_base_derivation_paths())
    }

    pub fn is_exploration_path_fixed(&self) -> bool {
        self.in_use.is_some()
            && (self.get_gui_exploration_path() == self.get_in_use_exploration_path())
    }

    pub fn is_exploration_depth_fixed(&self) -> bool {
        self.in_use.is_some()
            && (self.get_gui_exploration_depth() == self.get_in_use_exploration_depth())
    }

    pub fn is_data_dir_fixed(&self) -> bool {
        self.in_use.is_some() && (self.get_gui_data_dir() == self.get_in_use_data_dir())
    }

    pub fn is_selected_descriptors_fixed(&self) -> bool {
        self.in_use.is_some()
            && self.get_gui_p2pk() == self.get_in_use_p2pk()
            && self.get_gui_p2pkh() == self.get_in_use_p2pkh()
            && self.get_gui_p2wpkh() == self.get_in_use_p2wpkh()
            && self.get_gui_p2shwpkh() == self.get_in_use_p2shwpkh()
            && self.get_gui_p2tr() == self.get_in_use_p2tr()
    }

    pub fn is_mnemonic_fixed(&self) -> bool {
        self.in_use.is_some() && (self.get_gui_mnemonic() == self.get_in_use_mnemonic())
    }

    pub fn is_passphrase_fixed(&self) -> bool {
        self.in_use.is_some() && (self.get_gui_passphrase() == self.get_in_use_passphrase())
    }

    pub fn is_sweep_fixed(&self) -> bool {
        self.in_use.is_some() && (self.get_gui_sweep() == self.get_in_use_sweep())
    }

    pub fn is_base_derivation_paths_from_presets_fixed(&self) -> bool {
        self.in_use.is_some()
            && (self.get_gui_base_derivation_paths_from_presets()
                == self.get_in_use_base_derivation_paths_from_presets())
    }

    pub fn is_input_fixed(&self) -> bool {
        self.is_base_derivation_paths_fixed()
            && self.is_data_dir_fixed()
            && self.is_exploration_depth_fixed()
            && self.is_exploration_path_fixed()
            && self.is_passphrase_fixed()
            && self.is_mnemonic_fixed()
            && self.is_selected_descriptors_fixed()
            && self.is_sweep_fixed()
            && self.is_base_derivation_paths_from_presets_fixed()
    }
}

#[derive(Debug)]
pub struct ExplorerSettingFromGui {
    gui_base_derivation_paths: BaseDerivationPathsGuiData,
    gui_base_derivation_paths_from_presets: bool,
    gui_exploration_path: ExplorationPathGuiData,
    gui_sweep: bool,
    gui_exploration_depth: ExplorationDepthGuiData,
    gui_network: NetworkGuiData,
    gui_p2pk: bool,
    gui_p2pkh: bool,
    gui_p2wpkh: bool,
    gui_p2shwpkh: bool,
    gui_p2tr: bool,
    gui_data_dir: DataDirGuiData,
    gui_mnemonic: MnemonicGuiData,
    gui_passphrase: PassphraseGuiData,
}

impl Default for ExplorerSettingFromGui {
    fn default() -> Self {
        Self {
            gui_base_derivation_paths: BaseDerivationPathsGuiData::new("m".to_string()),
            gui_base_derivation_paths_from_presets: false,
            gui_exploration_path: ExplorationPathGuiData::new(DEFAULT_EXPLORATION_PATH.to_string()),
            gui_sweep: false,
            gui_exploration_depth: ExplorationDepthGuiData::new(
                DEFAULT_EXPLORATION_DEPTH.to_string(),
            ),
            gui_p2pk: true,
            gui_p2pkh: true,
            gui_p2wpkh: true,
            gui_p2shwpkh: true,
            gui_p2tr: true,
            gui_network: NetworkGuiData::new(bitcoin::Network::Bitcoin),
            gui_data_dir: DataDirGuiData::new("".to_string()),
            gui_mnemonic: MnemonicGuiData::new(),
            gui_passphrase: PassphraseGuiData::new("".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct BaseDerivationPathsGuiData {
    base_derivation_paths: String,
    sanity: bool,
}

impl GuiInput for BaseDerivationPathsGuiData {
    fn new(value: String) -> Self {
        let paths = value
            .split(';')
            .map(|path| path.trim().to_string())
            .collect::<Vec<_>>();
        let sanity = !paths
            .iter()
            .any(|path| DerivationPath::from_str(path).is_err());
        BaseDerivationPathsGuiData {
            base_derivation_paths: value,
            sanity,
        }
    }

    fn is_sane(&self) -> bool {
        self.sanity
    }

    fn get_value(&self) -> String {
        self.base_derivation_paths.to_owned()
    }
}

#[derive(Debug)]
pub struct ExplorationPathGuiData {
    exploration_path: String,
    sanity: bool,
}

impl GuiInput for ExplorationPathGuiData {
    fn new(value: String) -> Self {
        let exploration_path = value.trim().to_string();
        let sanity = ExplorationPath::new(None, &exploration_path, 1, false).is_ok();
        ExplorationPathGuiData {
            exploration_path,
            sanity,
        }
    }

    fn is_sane(&self) -> bool {
        self.sanity
    }

    fn get_value(&self) -> String {
        self.exploration_path.to_owned()
    }
}

#[derive(Debug)]
pub struct ExplorationDepthGuiData {
    exploration_depth: String,
    sanity: bool,
}

impl GuiInput for ExplorationDepthGuiData {
    fn new(value: String) -> Self {
        let exploration_depth = value.trim().to_string();
        let sanity = exploration_depth.parse::<u32>().is_ok();
        ExplorationDepthGuiData {
            exploration_depth,
            sanity,
        }
    }

    fn is_sane(&self) -> bool {
        self.sanity
    }

    fn get_value(&self) -> String {
        self.exploration_depth.to_owned()
    }
}

#[derive(Debug)]
pub struct NetworkGuiData {
    network: bitcoin::Network,
}

impl NetworkGuiData {
    fn new(value: bitcoin::Network) -> Self {
        NetworkGuiData { network: value }
    }

    fn get_value(&self) -> bitcoin::Network {
        self.network.to_owned()
    }
}

#[derive(Debug)]
pub struct DataDirGuiData {
    data_dir: String,
    sanity: bool,
}

impl GuiInput for DataDirGuiData {
    fn new(value: String) -> Self {
        let data_dir = value.trim().to_string();
        let path = PathBuf::from_str(&data_dir).unwrap();
        let sanity = path.canonicalize().is_ok()
            && path.canonicalize().unwrap().exists()
            && path.canonicalize().unwrap().is_dir();
        DataDirGuiData { data_dir, sanity }
    }

    fn is_sane(&self) -> bool {
        self.sanity
    }

    fn get_value(&self) -> String {
        self.data_dir.to_owned()
    }
}

#[derive(Debug)]
pub struct MnemonicGuiData {
    mnemonic: text_editor::Content,
    sanity: bool,
}

impl MnemonicGuiData {
    fn new() -> Self {
        MnemonicGuiData {
            mnemonic: text_editor::Content::new(),
            sanity: false,
        }
    }

    fn update(&mut self, action: text_editor::Action) {
        self.mnemonic.perform(action);
        self.sanity = bip39::Mnemonic::from_str(self.mnemonic.text().as_str()).is_ok();
    }

    fn is_sane(&self) -> bool {
        self.sanity
    }

    fn get_value(&self) -> String {
        self.mnemonic.text()
    }
}

#[derive(Debug)]
pub struct PassphraseGuiData {
    passphrase: String,
    sanity: bool,
}

impl GuiInput for PassphraseGuiData {
    fn new(value: String) -> Self {
        PassphraseGuiData {
            passphrase: value,
            sanity: true,
        }
    }

    fn is_sane(&self) -> bool {
        self.sanity
    }

    fn get_value(&self) -> String {
        self.passphrase.to_owned()
    }
}

#[derive(Debug)]
pub struct ExplorerSettingInUse {
    in_use_base_derivation_paths: Vec<String>,
    in_use_base_derivation_paths_from_presets: bool,
    in_use_exploration_path: String,
    in_use_sweep: bool,
    in_use_exploration_depth: u32,
    in_use_network: bitcoin::Network,
    in_use_selected_descriptors: Vec<CoveredDescriptors>,
    in_use_data_dir: String,
    in_use_mnemonic: String,
    in_use_passphrase: String,
}
