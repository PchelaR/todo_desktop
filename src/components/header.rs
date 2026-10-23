use crate::theme;
use crate::theme::tokens::{fonts, spacing};
use iced::font::Weight;
use iced::widget::text::LineHeight;
use iced::widget::{container, row, svg, text};
use iced::{Element, Font, Length, Pixels};

pub fn header<'a, M: 'a>() -> Element<'a, M> {
    container(
        row![
            svg(svg::Handle::from_memory(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/logo.svg"
            ))))
            .width(24)
            .height(24),
            text("TodoDesktop")
                .font(Font {
                    weight: Weight::Bold,
                    ..fonts::INTER
                })
                .line_height(LineHeight::Absolute(Pixels(24.0))),
        ]
        .spacing(spacing::SM),
    )
    .width(Length::Fill)
    .padding(spacing::MD)
    .style(theme::container::header)
    .into()
}
