use iced::{
    widget::{
        text, Button, Checkbox, Column, PickList, Row, Rule, Space, Text, TextEditor, TextInput,
    },
    Alignment, Font, Length,
};

use crate::{
    app_message::{
        setting_input_fixed::SettingInputFixedMessage,
        setting_input_in_gui::SettingInputInGuiMessage, AppMessage,
    },
    retriever_styles::{
        fix_button_style::FixButtonStyle, retriever_colors::BITCOIN_ORANGE_COLOR,
        sanity_check_light::SanityCheckLight,
    },
    RetrieverApp,
};

use super::common::sanity_checked_text_input;

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
        .push(Space::new(Length::Fill, 5))
        .push(third_row(app))
        .push(Space::new(Length::Fill, 5))
        .push(fourth_row(app))
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
        .push(mnemonic_block(app))
        .align_items(Alignment::Center)
        .into()
}

pub fn fourth_row(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
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
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
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
    if !app.explorer_setting_input.is_input_fixed()
        && app.explorer_setting_input.is_gui_input_sane()
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
            SettingInputFixedMessage::ExplorerSettingFixed,
        ))
        .into()
    } else {
        Button::new(text("Fix Settings").horizontal_alignment(iced::alignment::Horizontal::Center).vertical_alignment(iced::alignment::Vertical::Center))
            .width(150)
            .height(30)
            .style(iced::theme::Button::Custom(Box::new(FixButtonStyle)))
            .into()
    }
}

pub fn base_derivation_paths_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(sanity_checked_text_input(
            app,
            25,
            None,
            "base derivation paths:".to_string(),
            "".to_string(),
            app.explorer_setting_input.get_gui_base_derivation_paths(),
            Box::new(|base_derivation_paths| {
                AppMessage::SettingInputInGuiChanged(
                    SettingInputInGuiMessage::BaseDerivationPathsChanged(base_derivation_paths),
                )
            }),
            app.explorer_setting_input
                .is_gui_base_derivation_paths_sane(),
            app.explorer_setting_input.is_base_derivation_paths_fixed(),
        ))
        .push(Space::new(10, 10))
        .push(text("use presets:"))
        .push(Space::new(7, 10))
        .push(
            Checkbox::new(
                "",
                app.explorer_setting_input
                    .get_gui_base_derivation_paths_from_presets(),
            )
            .on_toggle(|base_derivation_paths_from_presets| {
                AppMessage::SettingInputInGuiChanged(
                    SettingInputInGuiMessage::BaseDerivationPathsFromPresetsChanged(
                        base_derivation_paths_from_presets,
                    ),
                )
            }),
        )
        .align_items(Alignment::Center)
        .into()
}

pub fn exploration_path_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(sanity_checked_text_input(
            app,
            25,
            None,
            "exploration path:".to_string(),
            "".to_string(),
            app.explorer_setting_input.get_gui_exploration_path(),
            Box::new(|exploration_path| {
                AppMessage::SettingInputInGuiChanged(
                    SettingInputInGuiMessage::ExplorationPathChanged(exploration_path),
                )
            }),
            app.explorer_setting_input.is_gui_exploration_path_sane(),
            app.explorer_setting_input.is_exploration_path_fixed(),
        ))
        .align_items(Alignment::Center)
        .into()
}

pub fn exploration_depth_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(sanity_checked_text_input(
            app,
            25,
            Some(60),
            "exploration depth:".to_string(),
            "".to_string(),
            app.explorer_setting_input.get_gui_exploration_depth(),
            Box::new(|exploration_depth| {
                AppMessage::SettingInputInGuiChanged(
                    SettingInputInGuiMessage::ExplorationDepthChanged(exploration_depth),
                )
            }),
            app.explorer_setting_input.is_gui_exploration_depth_sane(),
            app.explorer_setting_input.is_exploration_depth_fixed(),
        ))
        .align_items(Alignment::Center)
        .into()
}

pub fn sweep_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(text("sweep:"))
        .push(Space::new(7, 10))
        .push(
            Checkbox::new("", app.explorer_setting_input.get_gui_sweep()).on_toggle(|sweep| {
                AppMessage::SettingInputInGuiChanged(SettingInputInGuiMessage::SweepChanged(sweep))
            }),
        )
        .align_items(Alignment::Center)
        .into()
}

pub fn network_selection_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(Text::new("network:"))
        .push(Space::new(7, 10))
        .push(PickList::new(
            // ["Bitcoin", "Testnet", "Regtest", "Signet"],
            [
                bitcoin::Network::Bitcoin,
                bitcoin::Network::Testnet,
                bitcoin::Network::Regtest,
                bitcoin::Network::Signet,
            ],
            // Some("Bitcoin"),
            Some(app.explorer_setting_input.get_gui_network()),
            |network| {
                AppMessage::SettingInputInGuiChanged(SettingInputInGuiMessage::NetworkChanged(
                    network,
                ))
            },
        ))
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
        .push(Space::new(1, 10))
        .push(
            Button::new("")
                .height(50)
                .style(iced::theme::Button::Custom(Box::new(SanityCheckLight {
                    is_sane: app.explorer_setting_input.is_gui_mnemonic_sane(),
                    is_fixed: app.explorer_setting_input.is_mnemonic_fixed(),
                }))),
        )
        .align_items(Alignment::Center)
        .into()
}

pub fn passphrase_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(text("passphrase:"))
        .push(Space::new(7, 10))
        .push(
            TextInput::new("", &app.explorer_setting_input.get_gui_passphrase())
                .on_input(|passphrase| {
                    AppMessage::SettingInputInGuiChanged(
                        SettingInputInGuiMessage::PassphraseChanged(passphrase),
                    )
                })
                .width(Length::Fill),
        )
        .push(Space::new(1, 10))
        .push(
            Button::new("")
                .height(25)
                .style(iced::theme::Button::Custom(Box::new(SanityCheckLight {
                    is_sane: app.explorer_setting_input.is_gui_passphrase_sane(),
                    is_fixed: app.explorer_setting_input.is_passphrase_fixed(),
                }))),
        )
        .align_items(Alignment::Center)
        .into()
}
