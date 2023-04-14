use std::time::Duration;

use crate::data::{SolveTime, Penalty};

pub mod data;
pub mod application;

fn main() {
    println!("{}", SolveTime::new(Duration::new(124, 10_000_000), Some(Penalty::Dnf)));
}
