use std::time::Duration;

use iced::{Application, Settings};

use crate::{
    application::KubiaTimer,
    data::{Penalty, SolveTime},
};

pub mod application;
pub mod data;

fn main() -> iced::Result {
    println!(
        "{}",
        SolveTime::new(Duration::new(124, 10_000_000), Some(Penalty::Dnf))
    );
    KubiaTimer::run(Settings::default())
}
