use iced::{
    advanced::widget::Text,
    widget::{text, Button, Column, Row, Rule, Space},
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

pub fn run_row(
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
        // .push(Space::new(Length::Fill, 5))
        .padding(15)
        .align_items(iced::Alignment::Start)
        .into()
}

pub fn section_title(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Column::new()
        .push(
            Row::new()
                .push(
                    Text::new("RUN")
                        .size(17)
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .style(iced::theme::Text::Color(BITCOIN_ORANGE_COLOR)),
                )
                .push(Space::new(Length::Fill, 10))
                // .push(client_setting_fix_button(app))
                .align_items(Alignment::Center),
        )
        .push(Space::new(Length::Fill, 2))
        .push(Rule::horizontal(5))
        .into()
}

pub fn first_row(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(create_retriever_block(app))
        .push(Space::new(15, 10))
        .push(check_dump_file_block(app))
        .push(Space::new(15, 10))
        .push(populate_utxo_block(app))
        .push(Space::new(15, 10))
        .push(search_block(app))
        .push(Space::new(15, 10))
        .push(get_details_block(app))
        .push(Space::new(15, 10))
        .align_items(iced::Alignment::Center)
        .into()
}

pub fn create_retriever_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    if
    //app.retriever_setting.is_none()
    !app.is_retriever_built
        && app.bitcoincore_client_setting_input.is_input_fixed()
        && app.explorer_setting_input.is_input_fixed()
        && app.retriever_specific_setting_input.is_input_fixed()

    {
        Button::new(
            text("create retriever")
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .on_press(AppMessage::CreateRetriever)
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    } else {
        Button::new(
            text("create retriever")
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    }
}

pub fn check_dump_file_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    if app.retriever_setting.is_some() && app.is_retriever_built && !app.is_dump_file_ready {
        Button::new(
            text("check dump file")
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .on_press(AppMessage::PrepareDumpFile)
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    } else {
        Button::new(
            text("check dump file")
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    }
}

pub fn populate_utxo_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    if app.is_retriever_built && app.is_dump_file_ready && !app.is_utxo_set_ready {
        Button::new(
            text("populate database")
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .on_press(AppMessage::PopulateUtxoDB)
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    } else {
        Button::new(
            text("populate database")
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    }
}

pub fn search_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    if app.is_utxo_set_ready && app.finds.is_empty() {
        Button::new(
            text("search")
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .on_press(AppMessage::Search)
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    } else {
        Button::new(
            text("search")
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    }
}

pub fn get_details_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    if !app.finds.is_empty() {
        Button::new(
            text("get details")
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .on_press(AppMessage::GetDetails)
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    } else {
        Button::new(
            text("get details")
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    }
}
