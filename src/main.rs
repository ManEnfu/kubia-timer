#![allow(clippy::single_match)]

use iced::{window, Application};

use crate::gui::application::KTApplication;

pub mod data;
pub mod gui;

pub mod tangible;

fn main() -> iced::Result {
    env_logger::init();

    KTApplication::run(iced::Settings {
        id: Some("com.github.manenfu.KubiaTimer".to_string()),
        default_text_size: 16.0,
        window: window::Settings {
            size: (800, 600),
            ..Default::default()
        },
        ..Default::default()
    })
}
