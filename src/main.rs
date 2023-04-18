use std::time::Duration;

use iced::{Application, Settings};

use crate::{
    application::KubiaTimer,
    data::{Penalty, SolveTime},
};

pub mod application;
pub mod data;

fn main() -> iced::Result {
    env_logger::init();
    KubiaTimer::run(Settings::default())
}
