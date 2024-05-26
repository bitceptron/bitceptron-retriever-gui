use iced::{
    widget::{Button, Row, Space, Text, TextInput},
    Length,
};

use crate::{
    app_message::AppMessage, retriever_styles::sanity_check_light::SanityCheckLight, RetrieverApp,
};

pub fn sanity_checked_text_input(
    app: &RetrieverApp,
    light_height: u16,
    input_width: Option<u16>,
    title: String,
    placeholder: String,
    value: String,
    on_input_callback: Box<dyn Fn(String) -> AppMessage>,
    is_sane: bool,
    is_fixed: bool,
) -> iced::Element<'_, AppMessage> {
    Row::new()
        .push(Text::new(title))
        .push(Space::new(7, light_height))
        .push(
            TextInput::new(&placeholder, &value)
                .width(if input_width.is_some() {
                    input_width.unwrap().into()
                } else {
                    Length::Fill
                })
                .on_input(on_input_callback),
        )
        .push(Space::new(1, 10))
        .push(
            Button::new("")
                .height(light_height)
                .style(iced::theme::Button::Custom(Box::new(SanityCheckLight {
                    is_sane,
                    is_fixed,
                }))),
        )
        .align_items(iced::Alignment::Center)
        .into()
}
