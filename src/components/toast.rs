use iced::widget::{container, text};
use iced::{Element, Length};

use crate::theme;
use crate::theme::tokens::spacing;

pub fn toast<'a, M: 'a>(message: &'a str) -> Element<'a, M> {
    container(
        container(text(message))
            .width(Length::Fill)
            .padding(spacing::MD)
            .style(theme::container::error),
    )
    .width(Length::Fill)
    .align_bottom(Length::Fill)
    .padding(spacing::MD)
    .into()
}
