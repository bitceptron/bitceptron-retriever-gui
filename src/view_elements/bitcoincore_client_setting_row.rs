use iced::{
    widget::{
        column, container, horizontal_space, pane_grid::Line, row, text, text_input, Column, Container, Row, Rule, Space, Text
    },
    Length,
};

use crate::{app_message::AppMessage, RetrieverApp};

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
        .push(first_row(app))
        .push(Space::new(Length::Fill, 10))
        .push(second_row(app))
        .padding(15)
        .align_items(iced::Alignment::Start)
        .into()
}

pub fn section_title(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Column::new()
        .push(
            Text::new("bitcoincore settings")
                .size(21)
                .shaping(text::Shaping::Advanced),
        )
        .push(Rule::horizontal(5))
        .push(Space::new(Length::Fill, 10))
        .into()
}

pub fn first_row(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(text("bitcoincore url:"))
        .push(Space::new(10, 10))
        .push(text_input("", "127.0.0.1").line_height(1.).width(150))
        .push(Space::new(Length::Fill, 10))
        .push(text("bitcoincore rpc port:"))
        .push(Space::new(10, 10))
        .push(text_input("", "8332").line_height(1.).width(70))
        .push(Space::new(Length::Fill, 10))
        .push(text("bitcoincore timeout(seconds):"))
        .push(Space::new(10, 10))
        .push(text_input("", "6800").line_height(1.).width(70))
        .align_items(iced::Alignment::Center)
        .into()
}

pub fn second_row(app: &RetrieverApp) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(text("bitcoincore cookie path:"))
        .push(Space::new(10, 10))
        .push(
            text_input("", "Enter cookie path")
                .line_height(1.)
                .width(Length::Fill),
        )
        .into()
}
