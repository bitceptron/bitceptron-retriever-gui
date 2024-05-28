use iced::{
    advanced::widget::Text,
    widget::{text, Button, Column, Container, Row, Rule, Scrollable, Space},
    Alignment, Font, Length,
};

use crate::{
    app_message::{
        setting_input_fixed::SettingInputFixedMessage,
        setting_input_in_gui::SettingInputInGuiMessage, AppMessage,
    },
    retriever_styles::{
        fix_button_style::FixButtonStyle, outputs_style::OutputStyles,
        retriever_colors::BITCOIN_ORANGE_COLOR,
    },
    RetrieverApp,
};

use super::common::sanity_checked_text_input;

pub fn results_row(
    app: &RetrieverApp,
) -> iced::Element<
    '_,
    <RetrieverApp as iced::Application>::Message,
    <RetrieverApp as iced::Application>::Theme,
    iced::Renderer,
> {
    Column::new()
        // .push(section_title(app))
        .push(Space::new(Length::Fill, 5))
        .push(first_row(app))
        .push(Space::new(Length::Fill, 10))
        .padding(15)
        .align_items(iced::Alignment::Start)
        .into()
}

// pub fn section_title(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
//     Column::new()
//         .push(
//             Row::new()
//                 .push(
//                     Text::new("RUN")
//                         .size(17)
//                         .font(Font {
//                             weight: iced::font::Weight::Bold,
//                             ..Default::default()
//                         })
//                         .style(iced::theme::Text::Color(BITCOIN_ORANGE_COLOR)),
//                 )
//                 .push(Space::new(Length::Fill, 10))
//                 // .push(client_setting_fix_button(app))
//                 .align_items(Alignment::Center),
//         )
//         .push(Space::new(Length::Fill, 2))
//         .push(Rule::horizontal(5))
//         .into()
// }

pub fn first_row(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Column::new()
        .push(results_title_block(app))
        .push(results_block(app))
        .push(Space::new(15, 10))
        .push(errors_title_block(app))
        .push(errors_block(app))
        .push(Space::new(15, 10))
        // .push(populate_utxo_block(app))
        // .push(Space::new(15, 10))
        // .push(search_block(app))
        // .push(Space::new(15, 10))
        // .push(get_details_block(app))
        // .push(Space::new(15, 10))
        .align_items(iced::Alignment::Center)
        .into()
}

pub fn results_title_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Text::new("Results")
        .style(iced::theme::Text::Color(BITCOIN_ORANGE_COLOR))
        .width(Length::Fill)
        .horizontal_alignment(iced::alignment::Horizontal::Left)
        .vertical_alignment(iced::alignment::Vertical::Center)
        .into()
}

pub fn results_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Container::new(Scrollable::new(Text::new(format!(
        "{:?}",
        app.final_finds
    ))))
    .width(Length::Fill)
    .height(Length::FillPortion(2))
    .align_x(iced::alignment::Horizontal::Left)
    .align_y(iced::alignment::Vertical::Top)
    .style(iced::theme::Container::Custom(Box::new(OutputStyles)))
    .into()
}

pub fn errors_title_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Text::new("Errors")
        .style(iced::theme::Text::Color(BITCOIN_ORANGE_COLOR))
        .width(Length::Fill)
        .horizontal_alignment(iced::alignment::Horizontal::Left)
        .vertical_alignment(iced::alignment::Vertical::Center)
        .into()
}

pub fn errors_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Container::new(Scrollable::new(Text::new(format!(
        "{:?}",
        app.errors
    ))))
    .width(Length::Fill)
    .height(Length::FillPortion(1))
    .align_x(iced::alignment::Horizontal::Left)
    .align_y(iced::alignment::Vertical::Top)
    .style(iced::theme::Container::Custom(Box::new(OutputStyles)))
    .into()
}
