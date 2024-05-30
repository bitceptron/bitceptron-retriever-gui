use iced::{border::Radius, widget::button, Application, Border, Vector};

use crate::RetrieverApp;

use super::retriever_colors::{
    FIXED_BACKGROUND_COLOR, INSANE_BACKGROUND_COLOR, SANE_BACKGROUND_COLOR,
};

pub struct SanityCheckLight {
    pub is_sane: bool,
    pub is_fixed: bool,
}

impl button::StyleSheet for SanityCheckLight {
    type Style = <RetrieverApp as Application>::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector::new(0., 0.),
            background: if self.is_fixed {
                Some(iced::Background::Color(FIXED_BACKGROUND_COLOR))
            } else if self.is_sane {
                Some(iced::Background::Color(SANE_BACKGROUND_COLOR))
            } else {
                Some(iced::Background::Color(INSANE_BACKGROUND_COLOR))
            },
            border: Border {
                radius: Radius::from([0., 0., 0., 0.]),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
