use iced::Theme;
use iced::widget::button;

pub fn icon(theme: &Theme, status: button::Status) -> button::Style {
    let mut style = button::primary(theme, status);
    style.background = None;
    style
}
