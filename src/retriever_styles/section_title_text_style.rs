use iced::advanced::widget::text;

use super::retriever_colors::BITCOIN_ORANGE_COLOR;

pub struct SectionTitleTextStyle;

impl text::StyleSheet for SectionTitleTextStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: Some(BITCOIN_ORANGE_COLOR),
        }
    }
}
