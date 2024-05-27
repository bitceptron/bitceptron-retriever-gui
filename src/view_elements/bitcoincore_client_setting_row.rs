use iced::{
    advanced::widget::Text,
    widget::{text, Button, Column, Row, Rule, Space, TextInput},
    Alignment, Font, Length,
};

use crate::{
    app_message::{
        setting_input_fixed::SettingInputFixedMessage,
        setting_input_in_gui::SettingInputInGuiMessage, AppMessage,
    },
    retriever_styles::{fix_button_style::FixButtonStyle, retriever_colors::BITCOIN_ORANGE_COLOR},
    RetrieverApp,
};

use super::common::sanity_checked_text_input;

pub fn bitcoincore_client_setting_row(
    app: &RetrieverApp,
) -> iced::Element<
    '_,
    <RetrieverApp as iced::Application>::Message,
    <RetrieverApp as iced::Application>::Theme,
    iced::Renderer,
> {
    Column::new()
        .push(section_title(app))
        .push(Space::new(Length::Fill, 5))
        .push(first_row(app))
        .push(Space::new(Length::Fill, 10))
        .padding(15)
        .align_items(iced::Alignment::Start)
        .into()
}

pub fn section_title(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Column::new()
        .push(
            Row::new()
                .push(
                    Text::new("bitcoincore settings")
                        .size(17)
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .style(iced::theme::Text::Color(BITCOIN_ORANGE_COLOR)),
                )
                .push(Space::new(Length::Fill, 10))
                .push(client_setting_fix_button(app))
                .align_items(Alignment::Center),
        )
        .push(Space::new(Length::Fill, 2))
        .push(Rule::horizontal(5))
        .into()
}

pub fn first_row(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(bitcoincore_url_block(app))
        .push(Space::new(15, 10))
        .push(bitcoincore_rpc_port_block(app))
        .push(Space::new(15, 10))
        .push(bitcoincore_timeout_block(app))
        .push(Space::new(15, 10))
        .push(bitcoincore_cookie_path_block(app))
        .align_items(iced::Alignment::Center)
        .into()
}

pub fn bitcoincore_url_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    sanity_checked_text_input(
        app,
        25,
        Some(120),
        "url:".to_string(),
        "".to_string(),
        app.bitcoincore_client_setting_input.get_gui_url(),
        Box::new(|new_url| {
            AppMessage::SettingInputInGuiChanged(SettingInputInGuiMessage::BitcoincoreUrlChanged(
                new_url,
            ))
        }),
        app.bitcoincore_client_setting_input.is_gui_url_sane(),
        app.bitcoincore_client_setting_input.is_url_fixed(),
    )
}

pub fn bitcoincore_rpc_port_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    sanity_checked_text_input(
        app,
        25,
        Some(60),
        "rpc port:".to_string(),
        "".to_string(),
        app.bitcoincore_client_setting_input.get_gui_rpc_port(),
        Box::new(|new_rpc_port| {
            AppMessage::SettingInputInGuiChanged(
                SettingInputInGuiMessage::BitcoincoreRpcPortChanged(new_rpc_port),
            )
        }),
        app.bitcoincore_client_setting_input.is_gui_rpc_port_sane(),
        app.bitcoincore_client_setting_input.is_rpc_port_fixed(),
    )
}

pub fn bitcoincore_timeout_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    sanity_checked_text_input(
        app,
        25,
        Some(60),
        "timeout (seconds):".to_string(),
        "".to_string(),
        app.bitcoincore_client_setting_input.get_gui_timeout(),
        Box::new(|new_timeout| {
            AppMessage::SettingInputInGuiChanged(
                SettingInputInGuiMessage::BitcoincoreTimeoutChanged(new_timeout),
            )
        }),
        app.bitcoincore_client_setting_input.is_gui_timeout_sane(),
        app.bitcoincore_client_setting_input.is_timeout_fixed(),
    )
}

pub fn bitcoincore_cookie_path_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    sanity_checked_text_input(
        app,
        25,
        None,
        "cookie path:".to_string(),
        "".to_string(),
        app.bitcoincore_client_setting_input.get_gui_cookie_path(),
        Box::new(|new_cookie_path| {
            AppMessage::SettingInputInGuiChanged(
                SettingInputInGuiMessage::BitcoincoreCookiePathChanged(new_cookie_path),
            )
        }),
        app.bitcoincore_client_setting_input
            .is_gui_cookie_path_sane(),
        app.bitcoincore_client_setting_input.is_cookie_path_fixed(),
    )
}

pub fn client_setting_fix_button(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    if !app.bitcoincore_client_setting_input.is_input_fixed()
        && app.bitcoincore_client_setting_input.is_gui_input_sane()
    {
        Button::new(
            text("Fix Settings")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center),
        )
        .width(150)
        .height(30)
        .style(iced::theme::Button::Custom(Box::new(FixButtonStyle)))
        .on_press(AppMessage::SettingInputGotFixed(
            SettingInputFixedMessage::BitcoincoreClientSettingFixed,
        ))
        .into()
    } else {
        Button::new(text("Fix Settings").horizontal_alignment(iced::alignment::Horizontal::Center))
            .width(150)
            .height(30)
            .style(iced::theme::Button::Custom(Box::new(FixButtonStyle)))
            .into()
    }
}
