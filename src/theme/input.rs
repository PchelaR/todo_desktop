use iced::Theme;
use iced::widget::text_input;

use super::tokens::{colors, radius};

pub fn primary(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let palette = theme.palette();
    let mut style = text_input::default(theme, status);

    style.border.color = palette.primary;
    style.border.radius = radius::SMALL.into();
    style.placeholder = colors::GRAY;
    style
}
