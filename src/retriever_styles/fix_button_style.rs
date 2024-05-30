use iced::{
    border::Radius, widget::button, Application, Background, Border, Color, Shadow, Vector,
};

use crate::RetrieverApp;

use super::retriever_colors::{
    ACTIVE_FIX_BUTTON_COLOR, INACTIVE_FIX_BUTTON_COLOR, SANE_BACKGROUND_COLOR,
};

pub struct FixButtonStyle {
    pub is_fixed: bool,
}

impl button::StyleSheet for FixButtonStyle {
    type Style = <RetrieverApp as Application>::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector::new(2., 2.),
            background: Some(Background::Color(ACTIVE_FIX_BUTTON_COLOR)),
            text_color: Color::BLACK,
            border: Border {
                radius: Radius::from(5),
                ..Default::default()
            },
            shadow: Shadow::default(),
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: Vector::default(),
            background: if self.is_fixed {
                Some(Background::Color(SANE_BACKGROUND_COLOR))
            } else {
                Some(Background::Color(INACTIVE_FIX_BUTTON_COLOR))
            },
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}
