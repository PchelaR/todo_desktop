#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod components;
mod crypto;
mod storage;
mod theme;
mod todo;

use app::TodoApp;
use theme::tokens::fonts;

use iced::{Size, window};

fn main() -> iced::Result {
    iced::application(TodoApp::new, TodoApp::update, TodoApp::view)
        .title("TodoDesktop")
        .font(
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/fonts/Inter_18pt-Bold.ttf"
            ))
            .as_slice(),
        )
        .font(
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/fonts/Inter_18pt-Regular.ttf"
            ))
            .as_slice(),
        )
        .default_font(fonts::INTER)
        .antialiasing(true)
        .window(window::Settings {
            size: Size::new(360.0, 560.0),
            resizable: false,
            icon: window::icon::from_file_data(
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/source/icon_windows.png"
                )),
                None,
            )
            .ok(),
            ..Default::default()
        })
        .theme(|_: &TodoApp| theme::todo_theme())
        .run()
}
