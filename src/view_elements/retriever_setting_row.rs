use iced::{
    widget::{
        text, Button, Checkbox, Column, Row, Rule, Space, Text,
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

pub fn retriever_setting_row(
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
        .padding(15)
        .align_items(iced::Alignment::Start)
        .into()
}

pub fn first_row(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(covered_descriptors_block(app))
        .push(Space::new(15, 10))
        .push(datadir_block(app))
        .align_items(Alignment::Center)
        .into()
}

pub fn section_title(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Column::new()
        .push(
            Row::new()
                .push(
                    Text::new("retriever settings")
                        .size(17)
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .style(iced::theme::Text::Color(BITCOIN_ORANGE_COLOR)),
                )
                .push(Space::new(Length::Fill, 10))
                .push(retriever_specific_setting_fix_button(app))
                .align_items(Alignment::Center),
        )
        .push(Space::new(Length::Fill, 2))
        .push(Rule::horizontal(5))
        .into()
}

pub fn retriever_specific_setting_fix_button(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    if !app.retriever_specific_setting_input.is_input_fixed()
        && app.retriever_specific_setting_input.is_gui_input_sane()
    {
        Button::new(
            text("Fix Settings")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center),
        )
        .width(150)
        .height(30)
        .style(iced::theme::Button::Custom(Box::new(FixButtonStyle {is_fixed: app.retriever_specific_setting_input.is_input_fixed()})))
        .on_press(AppMessage::SettingInputGotFixed(
            SettingInputFixedMessage::RetrieverSettingFixed,
        ))
        .into()
    } else {
        Button::new(text("Fix Settings").horizontal_alignment(iced::alignment::Horizontal::Center).vertical_alignment(iced::alignment::Vertical::Center))
            .width(150)
            .height(30)
            .style(iced::theme::Button::Custom(Box::new(FixButtonStyle{is_fixed: app.retriever_specific_setting_input.is_input_fixed()})))
            .into()
    }
}

pub fn covered_descriptors_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(text("p2pk:"))
        .push(Space::new(5, 10))
        .push(
            Checkbox::new("", app.retriever_specific_setting_input.get_gui_p2pk()).on_toggle(|p2pk| {
                AppMessage::SettingInputInGuiChanged(
                    SettingInputInGuiMessage::P2pkInclusionChanged(p2pk),
                )
            }),
        )
        .push(text("p2pkh:"))
        .push(Space::new(5, 10))
        .push(
            Checkbox::new("", app.retriever_specific_setting_input.get_gui_p2pkh()).on_toggle(|p2pkh| {
                AppMessage::SettingInputInGuiChanged(
                    SettingInputInGuiMessage::P2pkhInclusionChanged(p2pkh),
                )
            }),
        )
        .push(text("p2wpk:"))
        .push(Space::new(5, 10))
        .push(
            Checkbox::new("", app.retriever_specific_setting_input.get_gui_p2wpkh()).on_toggle(|p2wpk| {
                AppMessage::SettingInputInGuiChanged(
                    SettingInputInGuiMessage::P2wpkhInclusionChanged(p2wpk),
                )
            }),
        )
        .push(text("p2shwpkh:"))
        .push(Space::new(5, 10))
        .push(
            Checkbox::new("", app.retriever_specific_setting_input.get_gui_p2shwpkh()).on_toggle(
                |p2shwpkh| {
                    AppMessage::SettingInputInGuiChanged(
                        SettingInputInGuiMessage::P2shwpkhInclusionChanged(p2shwpkh),
                    )
                },
            ),
        )
        .push(text("p2tr:"))
        .push(Space::new(5, 10))
        .push(
            Checkbox::new("", app.retriever_specific_setting_input.get_gui_p2tr()).on_toggle(|p2tr| {
                AppMessage::SettingInputInGuiChanged(
                    SettingInputInGuiMessage::P2trInclusionChanged(p2tr),
                )
            }),
        )
        .push(
            Button::new("")
                .height(25)
                .style(iced::theme::Button::Custom(Box::new(SanityCheckLight {
                    is_sane: app
                        .retriever_specific_setting_input
                        .is_gui_selected_descriptors_sane(),
                    is_fixed: app.retriever_specific_setting_input.is_selected_descriptors_fixed(),
                }))),
        )
        .align_items(Alignment::Center)
        .into()
}

pub fn datadir_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(sanity_checked_text_input(
            app,
            25,
            None,
            "data dir:".to_string(),
            "".to_string(),
            app.retriever_specific_setting_input.get_gui_data_dir(),
            Box::new(|data_dir| {
                AppMessage::SettingInputInGuiChanged(SettingInputInGuiMessage::DataDirChanged(
                    data_dir,
                ))
            }),
            app.retriever_specific_setting_input.is_gui_data_dir_sane(),
            app.retriever_specific_setting_input.is_data_dir_fixed(),
        ))
        .align_items(Alignment::Center)
        .into()
}
