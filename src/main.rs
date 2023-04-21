#![allow(clippy::single_match)]

use iced::{window, Application};

use crate::application::KubiaTimer;

pub mod application;
pub mod data;

pub mod tangible;

fn main() -> iced::Result {
    env_logger::init();

    KubiaTimer::run(iced::Settings {
        default_text_size: 18.0,
        window: window::Settings {
            size: (800, 600),
            ..Default::default()
        },
        ..Default::default()
    })
}
