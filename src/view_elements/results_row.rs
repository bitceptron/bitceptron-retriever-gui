use iced::{
    advanced::widget::Text,
    widget::{Column, Container, Scrollable, Space},
    Font, Length,
};

use crate::{
    app_message::AppMessage,
    retriever_styles::{outputs_style::OutputStyles, retriever_colors::BITCOIN_ORANGE_COLOR},
    RetrieverApp,
};

pub fn results_row(
    app: &RetrieverApp,
) -> iced::Element<
    '_,
    <RetrieverApp as iced::Application>::Message,
    <RetrieverApp as iced::Application>::Theme,
    iced::Renderer,
> {
    Column::new()
        .push(Space::new(Length::Fill, 5))
        .push(first_row(app))
        .push(Space::new(Length::Fill, 10))
        .padding(15)
        .align_items(iced::Alignment::Start)
        .into()
}

pub fn first_row(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Column::new()
        .push(results_title_block(app))
        .push(results_block(app))
        .push(Space::new(15, 10))
        .push(errors_title_block(app))
        .push(errors_block(app))
        .push(Space::new(15, 10))
        .align_items(iced::Alignment::Center)
        .into()
}

pub fn results_title_block(_app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Text::new("Results")
        .font(Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        })
        .style(iced::theme::Text::Color(BITCOIN_ORANGE_COLOR))
        .width(Length::Fill)
        .horizontal_alignment(iced::alignment::Horizontal::Left)
        .vertical_alignment(iced::alignment::Vertical::Center)
        .into()
}

// pub fn results_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
//     Container::new(Scrollable::new(Text::new(format!("{:?}", app.final_finds))).width(Length::Fill))
//         .width(Length::Fill)
//         .height(Length::FillPortion(2))
//         .align_x(iced::alignment::Horizontal::Left)
//         .align_y(iced::alignment::Vertical::Top)
//         .style(iced::theme::Container::Custom(Box::new(OutputStyles)))
//         .into()
// }

pub fn results_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    let text = if app.finds.is_empty() {
        "None".to_string()
    } else if !app.finds.is_empty() && app.final_finds.is_empty() {
        format!("{} ScriptPubkeys derived from you mnemonic was found. To see the details, press `get details` button.", app.finds.len())
    } else {
        format!("{:?}", app.final_finds)
    };
    Container::new(Scrollable::new(Text::new(text)).width(Length::Fill))
        .width(Length::Fill)
        .height(Length::FillPortion(2))
        .align_x(iced::alignment::Horizontal::Left)
        .align_y(iced::alignment::Vertical::Top)
        .style(iced::theme::Container::Custom(Box::new(OutputStyles)))
        .into()
}

pub fn errors_title_block(_app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Text::new("Errors")
        .font(Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        })
        .style(iced::theme::Text::Color(BITCOIN_ORANGE_COLOR))
        .width(Length::Fill)
        .horizontal_alignment(iced::alignment::Horizontal::Left)
        .vertical_alignment(iced::alignment::Vertical::Center)
        .into()
}

pub fn errors_block(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    let text = if app.errors.is_empty() {
        "None".to_string()
    } else {
        format!("{:#?}", app.errors)
    };
    Container::new(Scrollable::new(Text::new(text)).width(Length::Fill))
        .width(Length::Fill)
        .height(Length::FillPortion(1))
        .align_x(iced::alignment::Horizontal::Left)
        .align_y(iced::alignment::Vertical::Top)
        .style(iced::theme::Container::Custom(Box::new(OutputStyles)))
        .into()
}
