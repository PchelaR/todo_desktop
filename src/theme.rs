pub(crate) mod button;
pub(crate) mod checkbox;
pub(crate) mod container;
pub(crate) mod input;
pub(crate) mod tokens;

use iced::{Color, Theme, theme::Palette};

pub fn todo_theme() -> Theme {
    Theme::custom(
        "Todo theme".to_string(),
        Palette {
            background: Color::from_rgb8(255, 255, 255),
            text: Color::from_rgb8(50, 50, 50),
            primary: Color::from_rgb8(0, 146, 255),
            ..Palette::LIGHT
        },
    )
}
