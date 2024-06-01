use std::str::FromStr;

use bitceptron_retriever::{
    data::defaults::{DEFAULT_EXPLORATION_DEPTH, DEFAULT_EXPLORATION_PATH},
    explorer::{exploration_path::ExplorationPath, explorer_setting::ExplorerSetting},
};
use bitcoin::bip32::DerivationPath;

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
                in_use_exploration_path: self.get_gui_exploration_path(),
                in_use_sweep: self.get_gui_sweep(),
                in_use_exploration_depth: self.get_gui_exploration_depth().parse::<u32>().unwrap(),
                in_use_network: self.get_gui_network(),
                in_use_mnemonic: self.get_gui_mnemonic(),
                in_use_passphrase: self.get_gui_passphrase(),
            },
            false => return Err(GuiError::GuiInputIsInsane),
        };
        self.in_use = Some(in_use);
        Ok(())
    }

    pub fn to_explorer_setting(&self) -> ExplorerSetting {
        if !self.is_input_fixed() {
            panic!("Explorer setting output was called before fixing gui settings")
        }
        ExplorerSetting::new(
            self.get_in_use_mnemonic(),
            self.get_in_use_passphrase(),
            self.get_in_use_base_derivation_paths(),
            self.get_in_use_exploration_path(),
            self.get_in_use_exploration_depth(),
            self.get_in_use_network(),
            self.get_in_use_sweep(),
        )
    }

    pub fn set_base_derivation_paths_from_gui_input(&mut self, base_derivation_paths: String) {
        self.gui_input.gui_base_derivation_paths =
            BaseDerivationPathsGuiData::new(base_derivation_paths)
    }

    pub fn set_base_derivation_paths_from_presets_from_gui_input(
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

    pub fn set_mnemonic_from_gui_input(&mut self, mnemonic: String) {
        self.gui_input.gui_mnemonic = MnemonicGuiData::new(mnemonic)
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

    pub fn get_gui_mnemonic(&self) -> String {
        self.gui_input.gui_mnemonic.get_value()
    }

    pub fn get_gui_passphrase(&self) -> String {
        self.gui_input.gui_passphrase.get_value()
    }

    pub fn get_in_use_base_derivation_paths(&self) -> Vec<String> {
        match &self.in_use {
            Some(in_use) => in_use.in_use_base_derivation_paths.clone(),
            None => vec!["".to_string()],
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

    pub fn get_in_use_exploration_depth(&self) -> u32 {
        match &self.in_use {
            Some(in_use) => in_use.in_use_exploration_depth,
            None => 0,
        }
    }

    pub fn get_in_use_network(&self) -> bitcoin::Network {
        match &self.in_use {
            Some(in_use) => in_use.in_use_network,
            None => bitcoin::Network::Bitcoin,
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

    pub fn is_gui_mnemonic_sane(&self) -> bool {
        self.gui_input.gui_mnemonic.is_sane()
    }

    pub fn is_gui_passphrase_sane(&self) -> bool {
        self.gui_input.gui_passphrase.is_sane()
    }

    pub fn is_gui_input_sane(&self) -> bool {
        self.is_gui_base_derivation_paths_sane()
            && self.is_gui_exploration_depth_sane()
            && self.is_gui_exploration_path_sane()
            && self.is_gui_passphrase_sane()
            && self.is_gui_mnemonic_sane()
    }

    pub fn is_base_derivation_paths_fixed(&self) -> bool {
        self.in_use.is_some()
            && (self.get_gui_base_derivation_paths()
                == self.get_in_use_base_derivation_paths().join(";"))
    }

    pub fn is_exploration_path_fixed(&self) -> bool {
        self.in_use.is_some()
            && (self.get_gui_exploration_path() == self.get_in_use_exploration_path())
    }

    pub fn is_exploration_depth_fixed(&self) -> bool {
        self.in_use.is_some()
            && (self.get_gui_exploration_depth() == self.get_in_use_exploration_depth().to_string())
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

    pub fn is_network_fixed(&self) -> bool {
        self.in_use.is_some() && (self.get_gui_network() == self.get_in_use_network())
    }

    pub fn is_input_fixed(&self) -> bool {
        self.is_base_derivation_paths_fixed()
            && self.is_exploration_depth_fixed()
            && self.is_exploration_path_fixed()
            && self.is_passphrase_fixed()
            && self.is_mnemonic_fixed()
            && self.is_network_fixed()
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
            gui_network: NetworkGuiData::new(bitcoin::Network::Bitcoin),
            gui_mnemonic: MnemonicGuiData::new("".to_string()),
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
        let sanity = exploration_depth.parse::<u32>().is_ok()
            && exploration_depth.parse::<u32>().unwrap() >= 1;
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
pub struct MnemonicGuiData {
    mnemonic: String,
    sanity: bool,
}

impl MnemonicGuiData {
    fn new(mnemonic: String) -> Self {
        let mnemonic = mnemonic.trim().to_string();
        let sanity = bip39::Mnemonic::from_str(mnemonic.as_str()).is_ok();
        MnemonicGuiData {
            mnemonic,
            sanity,
        }
    }

    fn is_sane(&self) -> bool {
        self.sanity
    }

    fn get_value(&self) -> String {
        self.mnemonic.clone()
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
    in_use_mnemonic: String,
    in_use_passphrase: String,
}
