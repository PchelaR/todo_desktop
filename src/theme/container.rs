use crate::theme::tokens::radius;
use iced::widget::container;
use iced::{Border, Theme};

pub fn header(theme: &Theme) -> container::Style {
    let palette = theme.palette();
    container::Style {
        background: Some(palette.primary.into()),
        text_color: Some(palette.background),
        ..container::Style::default()
    }
}

pub fn error(theme: &Theme) -> container::Style {
    let palette = theme.palette();
    container::Style {
        background: Some(palette.background.into()),
        text_color: Some(palette.danger),
        border: Border {
            color: palette.danger,
            width: 1.0,
            radius: radius::SMALL.into(),
        },
        ..container::Style::default()
    }
}
