use crate::app::Message;
use crate::theme;
use crate::theme::tokens::{colors, spacing};
use crate::todo::TodoItem;
use iced::alignment::Vertical;
use iced::widget::text::LineHeight;
use iced::widget::{button, checkbox, row, svg, text};
use iced::{Element, Length, Pixels};

pub fn task_row(task: &TodoItem) -> Element<'_, Message> {
    let id = task.id;
    let color = if task.done { Some(colors::GRAY) } else { None };

    row![
        checkbox(task.done)
            .size(24)
            .on_toggle(move |_| Message::ToggleTask(id))
            .style(theme::checkbox::primary),
        text(&task.description)
            .width(Length::Fill)
            .line_height(LineHeight::Absolute(Pixels(24.0)))
            .color_maybe(color),
        button(
            svg(svg::Handle::from_memory(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/close.svg"
            ))))
            .width(24)
            .height(24)
        )
        .style(theme::button::icon)
        .padding(0)
        .on_press(Message::DeleteTask(id)),
    ]
    .spacing(spacing::MD)
    .align_y(Vertical::Top)
    .into()
}
