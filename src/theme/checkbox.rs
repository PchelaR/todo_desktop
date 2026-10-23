use crate::theme::tokens::radius;
use iced::Theme;
use iced::widget::checkbox;

pub fn primary(theme: &Theme, status: checkbox::Status) -> checkbox::Style {
    let palette = theme.palette();
    let mut style = checkbox::primary(theme, status);

    style.background = palette.background.into();
    style.icon_color = palette.primary;
    style.border.color = palette.primary;
    style.border.width = 1.0;
    style.border.radius = radius::SMALL.into();
    style
}
