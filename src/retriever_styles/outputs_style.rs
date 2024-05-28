use iced::{widget::container, Application, Border, Color, Shadow};

use crate::RetrieverApp;

pub struct OutputStyles;

impl container::StyleSheet for OutputStyles {
    type Style = <RetrieverApp as Application>::Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::from_rgb8(0, 255, 0)),
            background: Some(iced::Background::Color(Color::BLACK)),
            border: Border { color: Color::WHITE, ..Default::default() },
            shadow: Shadow{ ..Default::default() },
        }
    }
}
