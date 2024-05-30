use iced::{
    advanced::widget::Text,
    widget::{ text, Button, Column, Row, Rule, Space},
    Alignment, Font, Length,
};

use crate::{
    app_message::AppMessage,
    retriever_styles::{
        retriever_colors::BITCOIN_ORANGE_COLOR, stop_button_style::StopButtonStyle,
    },
    RetrieverApp,
};

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
        .padding(15)
        .align_items(iced::Alignment::Start)
        .into()
}

pub fn section_title(_app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
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
                .align_items(Alignment::Center),
        )
        .push(Space::new(Length::Fill, 2))
        .push(Rule::horizontal(5))
        .into()
}

pub fn first_row(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(create_new_dump_file_block(app))
        .push(Space::new(15, 10))
        .push(create_or_use_dump_file_block(app))
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

pub fn create_new_dump_file_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    if app.bitcoincore_client_setting_input.is_input_fixed()
        && app.retriever_specific_setting_input.is_input_fixed()
    {
        Button::new(
            text("new dump file")
                .font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .on_press(AppMessage::CreateClientForNewDumpFileAndThenCreate)
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    } else {
        Button::new(
            text("new dump file")
                .font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    }
}

pub fn create_or_use_dump_file_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    if app.bitcoincore_client_setting_input.is_input_fixed()
        && app.retriever_specific_setting_input.is_input_fixed()
    {
        Button::new(
            text("use/create dump file")
                .font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .on_press(AppMessage::CreateClientForDumpFileAndThenPrepare)
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    } else {
        Button::new(
            text("use/create dump file")
                .font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    }
}

pub fn populate_utxo_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    if app.is_dump_file_ready && !app.is_populating_in_progress && !app.is_search_in_progress {
        Button::new(
            text("populate database")
                .font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .on_press(AppMessage::PopulateUtxoDB)
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    } else if app.is_populating_in_progress {
        Button::new(
            text("stop populating")
                .font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .on_press(AppMessage::StopPopulatingUtxoDB)
        .style(iced::theme::Button::Custom(Box::new(StopButtonStyle)))
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    } else {
        Button::new(
            text("populate database")
                .font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    }
}

pub fn search_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    if !app.uspk_set.is_empty()
        && app.explorer_setting_input.is_input_fixed()
        && app.retriever_specific_setting_input.is_input_fixed()
        && !app.is_search_in_progress
        && !app.is_populating_in_progress
    {
        Button::new(
            text("search").font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .on_press(AppMessage::Search)
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    } else if app.is_search_in_progress {
        Button::new(
            text("stop search").font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .on_press(AppMessage::StopSearch)
        .style(iced::theme::Button::Custom(Box::new(StopButtonStyle)))
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    } else {
        Button::new(
            text("search").font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    }
}

pub fn get_details_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    if !app.finds.is_empty() && app.bitcoincore_client_setting_input.is_input_fixed() {
        Button::new(
            text("get details").font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .on_press(AppMessage::CreateClientForGettingDetailsAndThenGet)
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    } else {
        Button::new(
            text("get details").font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .height(30)
        .width(Length::FillPortion(1))
        .into()
    }
}
