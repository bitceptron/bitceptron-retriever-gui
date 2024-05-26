use iced::{
    widget::{
        text,
        Button, Checkbox, Column, PickList, Row, Rule, Space, Text, TextEditor, TextInput,
    }, Alignment, Font, Length
};

use crate::{
    app_message::{
        setting_input_fixed::SettingInputFixedMessage,
        setting_input_in_gui::SettingInputInGuiMessage, AppMessage,
    },
    retriever_styles::{fix_button_style::FixButtonStyle, retriever_colors::BITCOIN_ORANGE_COLOR},
    RetrieverApp,
};

pub fn exploration_setting_row(
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
        .push(Space::new(Length::Fill, 5))
        .push(second_row(app))
        .push(Space::new(Length::Fill, 5))
        .push(third_row(app))
        .push(Space::new(Length::Fill, 5))
        .push(fourth_row(app))
        .push(Space::new(Length::Fill, 5))
        .push(fifth_row(app))
        .padding(15)
        .align_items(iced::Alignment::Start)
        .into()
}

pub fn first_row(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(base_derivation_paths_block(app))
        .align_items(Alignment::Center)
        .into()
}

pub fn second_row(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(exploration_path_block(app))
        .push(Space::new(7, 10))
        .push(exploration_depth_block(app))
        .push(Space::new(15, 10))
        .push(network_selection_block(app))
        .push(Space::new(15, 10))
        .push(sweep_block(app))
        .align_items(Alignment::Center)
        .into()
}

pub fn third_row(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(covered_descriptors_block(app))
        .push(Space::new(15, 10))
        .push(datadir_block(app))
        .align_items(Alignment::Center)
        .into()
}

pub fn fourth_row(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(mnemonic_block(app))
        .align_items(Alignment::Center)
        .into()
}

pub fn fifth_row(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(passphrase_block(app))
        .align_items(Alignment::Center)
        .into()
}

pub fn section_title(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Column::new()
        .push(
            Row::new()
                .push(
                    Text::new("exploration settings")
                        .size(17)
                        .font(Font{  weight: iced::font::Weight::Bold, ..Default::default() })
                        .style(iced::theme::Text::Color(BITCOIN_ORANGE_COLOR)),
                )
                .push(Space::new(Length::Fill, 10))
                .push(exploration_setting_fix_button(app))
                .align_items(Alignment::Center),
        )
        .push(Space::new(Length::Fill, 2))
        .push(Rule::horizontal(5))
        .into()
}

pub fn exploration_setting_fix_button(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    if !app.bitcoincore_client_setting_input.is_gui_input_fixed()
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

pub fn base_derivation_paths_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(Text::new("base derivation paths:"))
        .push(Space::new(7, 10))
        .push(TextInput::new("Enter base derivation paths", "m").width(Length::Fill))
        .push(Space::new(10, 10))
        .push(text("use presets:"))
        .push(Space::new(7, 10))
        .push(Checkbox::new("", false))
        .align_items(Alignment::Center)
        .into()
}

pub fn exploration_path_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(Text::new("exploration path:"))
        .push(Space::new(7, 10))
        .push(TextInput::new("Enter exploration path", "m").width(Length::Fill))
        .align_items(Alignment::Center)
        .into()
}

pub fn exploration_depth_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(Text::new("exploration depth:"))
        .push(Space::new(7, 10))
        .push(TextInput::new("Enter exploration depth", "50").width(60))
        .align_items(Alignment::Center)
        .into()
}

pub fn sweep_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(text("sweep:"))
        .push(Space::new(7, 10))
        .push(Checkbox::new("", false))
        .align_items(Alignment::Center)
        .into()
}

pub fn network_selection_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(Text::new("network:"))
        .push(Space::new(7, 10))
        .push(PickList::new(
            ["Bitcoin", "Testnet", "Regtest", "Signet"],
            Some("Bitcoin"),
            |network| {
                AppMessage::SettingInputInGuiChanged(
                    SettingInputInGuiMessage::ExplorerNetworkChanged(network.to_string()),
                )
            },
        ))
        .align_items(Alignment::Center)
        .into()
}

pub fn covered_descriptors_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(text("p2pk:"))
        .push(Space::new(5, 10))
        .push(Checkbox::new("", false))
        .push(text("p2pkh:"))
        .push(Space::new(5, 10))
        .push(Checkbox::new("", false))
        .push(text("p2wpk:"))
        .push(Space::new(5, 10))
        .push(Checkbox::new("", false))
        .push(text("p2shwpkh:"))
        .push(Space::new(5, 10))
        .push(Checkbox::new("", false))
        .push(text("p2tr:"))
        .push(Space::new(5, 10))
        .push(Checkbox::new("", false))
        .align_items(Alignment::Center)
        .into()
}

pub fn datadir_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(text("data directory:"))
        .push(Space::new(7, 10))
        .push(TextInput::new("Enter data directory", ""))
        .align_items(Alignment::Center)
        .into()
}

pub fn mnemonic_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(text("mnemonic:"))
        .push(Space::new(21, 10))
        .push(
            TextEditor::new(&app.mnemonic_content)
                .height(50)
                .on_action(|action| {
                    AppMessage::SettingInputInGuiChanged(SettingInputInGuiMessage::MnemonicChanged(
                        action,
                    ))
                }),
        )
        .align_items(Alignment::Center)
        .into()
}

pub fn passphrase_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(text("passphrase:"))
        .push(Space::new(7, 10))
        .push(
            TextInput::new("Enter passphrase", "")
                .secure(true)
                .width(Length::Fill),
        )
        .align_items(Alignment::Center)
        .into()
}
