use app_message::{setting_input_in_gui::SettingInputInGuiMessage, AppMessage};
use iced::{
    executor,
    widget::{text_editor, Column, Space},
    Application, Command, Length,
};
use inputs::{bitcoincore_client::BitcoincoreClientInput, explorer::ExplorerInput};
use view_elements::{
    bitcoincore_client_setting_row, common::sanity_checked_text_input, exploration_setting_row,
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
                    self.bitcoincore_client_setting_input.set_url(new_url)
                }
                SettingInputInGuiMessage::BitcoincoreRpcPortChanged(new_rpc_port) => self
                    .bitcoincore_client_setting_input
                    .set_rpc_port(new_rpc_port),
                SettingInputInGuiMessage::BitcoincoreTimeoutChanged(new_timeout) => self
                    .bitcoincore_client_setting_input
                    .set_timeout(new_timeout),
                SettingInputInGuiMessage::BitcoincoreCookiePathChanged(new_cookie_path) => self
                    .bitcoincore_client_setting_input
                    .set_cookie_path(new_cookie_path),
                SettingInputInGuiMessage::ExplorerNetworkChanged(_) => {},
                SettingInputInGuiMessage::MnemonicChanged(action) => self.mnemonic_content.perform(action),
            },
            AppMessage::SettingInputGotFixed(input_fixed) => match input_fixed {
                app_message::setting_input_fixed::SettingInputFixedMessage::BitcoincoreClientSettingFixed => {let _ = self.bitcoincore_client_setting_input.gui_to_in_use();},
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
