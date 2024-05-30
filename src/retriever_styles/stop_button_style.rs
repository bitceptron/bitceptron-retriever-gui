use iced::{widget::button, Application, Background, Color, Vector};

use crate::RetrieverApp;

pub struct StopButtonStyle;

impl button::StyleSheet for StopButtonStyle {
    type Style = <RetrieverApp as Application>::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector::new(0., 0.),
            background: Some(Background::Color(Color::from_rgb8(255, 0, 0))),
            ..Default::default()
        }
    }
}
