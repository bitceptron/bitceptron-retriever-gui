// use iced::{widget::text_input, Background, Border, Color};

// use crate::RetrieverApp;

// use super::retriever_colors::{
//     FIXED_BACKGROUND_COLOR, INACTIVE_BACKGROUND_COLOR, INSANE_BACKGROUND_COLOR,
//     SANE_BACKGROUND_COLOR,
// };

// pub struct SanityCheckedTextInput {
//     pub is_sane: bool,
//     pub is_fixed: bool,
// }

// impl text_input::StyleSheet for SanityCheckedTextInput {
//     type Style = <RetrieverApp as iced::Application>::Theme;

//     fn active(&self, style: &Self::Style) -> text_input::Appearance {
//         text_input::Appearance {
//             background: if self.is_fixed {
//                 Background::Color(FIXED_BACKGROUND_COLOR)
//             } else if self.is_sane {
//                 Background::Color(SANE_BACKGROUND_COLOR)
//             } else {
//                 Background::Color(INSANE_BACKGROUND_COLOR)
//             },
//             border: Border::default(),
//             icon_color: Color::WHITE,
//         }
//     }

//     fn focused(&self, style: &Self::Style) -> text_input::Appearance {
//         text_input::Appearance {
//             background: if self.is_fixed {
//                 Background::Color(FIXED_BACKGROUND_COLOR)
//             } else if self.is_sane {
//                 Background::Color(SANE_BACKGROUND_COLOR)
//             } else {
//                 Background::Color(INSANE_BACKGROUND_COLOR)
//             },
//             border: Border::default(),
//             icon_color: Color::WHITE,
//         }
//     }

//     fn placeholder_color(&self, style: &Self::Style) -> Color {
//         Color::WHITE
//     }

//     fn value_color(&self, style: &Self::Style) -> Color {
//         Color::BLACK
//     }

//     fn disabled_color(&self, style: &Self::Style) -> Color {
//         Color::WHITE
//     }

//     fn selection_color(&self, style: &Self::Style) -> Color {
//         Color::WHITE
//     }

//     fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
//         text_input::Appearance {
//             background: Background::Color(INACTIVE_BACKGROUND_COLOR),
//             border: Border::default(),
//             icon_color: Color::WHITE,
//         }
//     }
// }
