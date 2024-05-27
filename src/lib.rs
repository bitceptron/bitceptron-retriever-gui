use app_message::{setting_input_in_gui::SettingInputInGuiMessage, AppMessage};
use iced::{
    executor,
    widget::{text_editor, Column, Space},
    Application, Command, Length,
};
use inputs::{bitcoincore_client::BitcoincoreClientInput, explorer::ExplorerInput};
use view_elements::{
    bitcoincore_client_setting_row, exploration_setting_row,
};

pub mod app_message;
pub mod gui_error;
pub mod inputs;
pub mod retriever_styles;
pub mod status;
pub mod view_elements;

#[derive(Debug, Default)]
pub struct RetrieverApp {
    bitcoincore_client_setting_input: BitcoincoreClientInput,
    explorer_setting_input: ExplorerInput,
    mnemonic_content: text_editor::Content,
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
                SettingInputInGuiMessage::BaseDerivationPathsFromPresetsChanged(base_derivation_paths_from_presets) => self.explorer_setting_input.set_gui_base_derivation_paths_from_presets_from_gui_input(base_derivation_paths_from_presets),
                SettingInputInGuiMessage::ExplorationPathChanged(exploration_path) => self.explorer_setting_input.set_exploration_path_from_gui_input(exploration_path),
                SettingInputInGuiMessage::SweepChanged(sweep) => self.explorer_setting_input.set_sweep_from_gui_input(sweep),
                SettingInputInGuiMessage::ExplorationDepthChanged(exploration_dept) => self.explorer_setting_input.set_exploration_depth_from_gui_input(exploration_dept),
                SettingInputInGuiMessage::P2pkInclusionChanged(p2pk_inclusion) => self.explorer_setting_input.set_p2pk_inclusion_from_gui_input(p2pk_inclusion),
                SettingInputInGuiMessage::P2pkhInclusionChanged(p2pkh_inclusion) => self.explorer_setting_input.set_p2pkh_inclusion_from_gui_input(p2pkh_inclusion),
                SettingInputInGuiMessage::P2wpkhInclusionChanged(p2wpkh_inclusion) => self.explorer_setting_input.set_p2wpkh_inclusion_from_gui_input(p2wpkh_inclusion),
                SettingInputInGuiMessage::P2shwpkhInclusionChanged(p2shwpkh_inclusion) => self.explorer_setting_input.set_p2shwpkh_inclusion_from_gui_input(p2shwpkh_inclusion),
                SettingInputInGuiMessage::P2trInclusionChanged(p2tr_inclusion) => self.explorer_setting_input.set_p2tr_inclusion_from_gui_input(p2tr_inclusion),
                SettingInputInGuiMessage::DataDirChanged(data_dir) => self.explorer_setting_input.set_data_dir_from_gui_input(data_dir),
                SettingInputInGuiMessage::PassphraseChanged(passphrase) => self.explorer_setting_input.set_passphrase_from_gui_input(passphrase),
            },
            AppMessage::SettingInputGotFixed(input_fixed) => match input_fixed {
                app_message::setting_input_fixed::SettingInputFixedMessage::BitcoincoreClientSettingFixed => {let _ = self.bitcoincore_client_setting_input.gui_to_in_use();},
                app_message::setting_input_fixed::SettingInputFixedMessage::ExplorerSettingFixed => {let _ = self.explorer_setting_input.gui_to_in_use();},
            },
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        Column::new()
            .push(bitcoincore_client_setting_row(self))
            .push(Space::new(Length::Fill, 0))
            .push(exploration_setting_row(self))
            .into()
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::TokyoNight
    }
}
