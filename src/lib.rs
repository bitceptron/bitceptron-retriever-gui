use std::sync::{Arc, Mutex};

use app_message::{setting_input_in_gui::SettingInputInGuiMessage, AppMessage};
use app_status::AppStatus;
use bitceptron_retriever::{
    client::client_setting::ClientSetting, error::RetrieverError,
    explorer::explorer_setting::ExplorerSetting, retriever::Retriever, setting::RetrieverSetting,
};
use iced::{
    executor,
    widget::{text_editor, Column, Space},
    Application, Command, Length,
};
use inputs::{
    bitcoincore_client::BitcoincoreClientInput, explorer::ExplorerInput,
    retriever_specific::RetrieverSpecificInput,
};
use run_functions::create_retriever_setting;
use view_elements::{
    bitcoincore_client_setting_row, exploration_setting_row, retriever_setting_row,
    run_row::run_row,
};

pub mod app_message;
pub mod app_status;
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
    // bitcoincore_client_setting: Option<ClientSetting>,
    // explorer_setting: Option<ExplorerSetting>,
    retriever_setting: Option<RetrieverSetting>,
    // retriever: Arc<Mutex<Option<Retriever>>>,
    retriever: Arc<Mutex<Retriever>>,
    errors: Vec<Arc<RetrieverError>>,
    app_status: AppStatus,
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
            // bitcoincore_client_setting: Default::default(),
            // explorer_setting: Default::default(),
            retriever_setting: None,
            // retriever: Arc::new(Mutex::new(None)),
            retriever: Arc::new(Mutex::new(Default::default())),
            errors: vec![],
            app_status: AppStatus::Empty,
            is_retriever_built: false,
            is_dump_file_ready: false,
            is_utxo_set_ready: false,
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
                    // self.bitcoincore_client_setting = Some(self.bitcoincore_client_setting_input.to_client_setting());
                },
                app_message::setting_input_fixed::SettingInputFixedMessage::ExplorerSettingFixed => {
                    let _ = self.explorer_setting_input.gui_to_in_use();
                    // self.explorer_setting = Some(self.explorer_setting_input.to_explorer_setting());
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

                Command::perform(self.retriever.lock().unwrap().check_for_dump_in_data_dir_or_create_dump_file(), |dump_result| {
                    match dump_result {
                        Ok(_) => AppMessage::DumpFileDone,
                        Err(e) => AppMessage::Error(Arc::new(e)),
                    }
                });
            },
            AppMessage::Search => todo!(),
            AppMessage::RetrieverCreated(retriever) => {
                self.retriever = Arc::new(Mutex::new(retriever));
                self.is_retriever_built = true;
            },
            AppMessage::Error(e) => self.errors.push(e),
            AppMessage::None => {},
            AppMessage::DumpFileDone => self.is_dump_file_ready = true,
            AppMessage::PopulateUtxoDB => todo!(),
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        Column::new()
            .push(bitcoincore_client_setting_row(self))
            // .push(Space::new(Length::Fill, 0))
            .push(exploration_setting_row(self))
            .push(retriever_setting_row(self))
            .push(run_row(self))
            .into()
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::TokyoNight
    }
}
