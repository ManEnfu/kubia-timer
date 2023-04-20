use std::{iter, slice};

use super::*;

#[derive(Debug, Clone)]
pub struct SessionEntry {
    pub solve: Solve,
    pub mo3: Option<SolveTime>,
    pub ao5: Option<SolveTime>,
    pub ao12: Option<SolveTime>,
}

impl SolvesSeq for &[SessionEntry] {
    fn mean_of_n(&self) -> Option<SolveTime> {
        let len = self.len() as u32;
        if len == 0 {
            return None;
        }

        let sum: SolveTime = self.iter().map(|se| se.solve.time).sum();
        Some(sum / len)
    }

    fn average_of_n(&self) -> Option<SolveTime> {
        let len = self.len() as u32;
        if len < 3 {
            return None;
        }

        let it = self.iter().map(|se| se.solve.time).enumerate();

        let (imax, _max) = it.clone().max_by_key(|&(_, st)| st)?;
        let (imin, _min) = it.clone().min_by_key(|&(_, st)| st)?;
        let sum = it.fold(SolveTime::default(), |acc, (i, st)| {
            if i != imax && i != imin {
                acc + st
            } else {
                acc
            }
        });
        Some(sum / (len - 2))
    }
}

#[derive(Clone)]
pub struct Session {
    entries: Vec<SessionEntry>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_solve(&mut self, solve: Solve) {
        self.entries.push(SessionEntry {
            solve,
            mo3: None,
            ao5: None,
            ao12: None,
        });

        self.update_statistics_last();
    }

    pub fn iter(&self) -> slice::Iter<SessionEntry> {
        self.entries.iter()
    }

    pub fn get_n_solves(&self) -> usize {
        self.entries.len()
    }

    pub fn get_solve(&self, index: usize) -> Option<&Solve> {
        self.entries.get(index).map(|se| &se.solve)
    }

    pub fn get_mo3(&self, index: usize) -> Option<SolveTime> {
        if self.entries.len() >= 3 {
            self.entries.get(index).and_then(|se| se.mo3)
        } else {
            None
        }
    }

    pub fn get_ao5(&self, index: usize) -> Option<SolveTime> {
        if self.entries.len() >= 5 {
            self.entries.get(index).and_then(|se| se.ao5)
        } else {
            None
        }
    }

    pub fn get_ao12(&self, index: usize) -> Option<SolveTime> {
        if self.entries.len() >= 12 {
            self.entries.get(index).and_then(|se| se.ao12)
        } else {
            None
        }
    }

    pub fn best_mo3(&self) -> Option<SolveTime> {
        self.entries.iter().map(|se| se.mo3).min().and_then(|st| st)
    }

    pub fn best_ao5(&self) -> Option<SolveTime> {
        self.entries.iter().map(|se| se.ao5).min().and_then(|st| st)
    }

    pub fn best_ao12(&self) -> Option<SolveTime> {
        self.entries
            .iter()
            .map(|se| se.ao12)
            .min()
            .and_then(|st| st)
    }

    pub fn last_solve(&self) -> Option<&Solve> {
        self.entries.last().map(|se| &se.solve)
    }

    pub fn last_solve_mut(&mut self) -> Option<&mut Solve> {
        self.entries.last_mut().map(|se| &mut se.solve)
    }

    pub fn last_mo3(&self) -> Option<SolveTime> {
        self.entries.last().and_then(|se| se.mo3)
    }

    pub fn last_ao5(&self) -> Option<SolveTime> {
        self.entries.last().and_then(|se| se.ao5)
    }

    pub fn last_ao12(&self) -> Option<SolveTime> {
        self.entries.last().and_then(|se| se.ao12)
    }

    fn compute_mo3(&mut self, index: usize) -> Option<SolveTime> {
        if self.entries.len() >= 3 {
            self.entries
                .get(index - 2..index + 1)
                .and_then(|solves| solves.mean_of_n())
        } else {
            None
        }
    }

    fn compute_ao5(&mut self, index: usize) -> Option<SolveTime> {
        if self.entries.len() >= 5 {
            self.entries
                .get(index - 4..index + 1)
                .and_then(|solves| solves.average_of_n())
        } else {
            None
        }
    }

    fn compute_ao12(&mut self, index: usize) -> Option<SolveTime> {
        if self.entries.len() >= 12 {
            self.entries
                .get(index - 11..index + 1)
                .and_then(|solves| solves.average_of_n())
        } else {
            None
        }
    }

    pub fn update_statistics(&mut self, index: usize) {
        let len = self.entries.len();
        for i in index..len.min(index + 3) {
            self.update_mo3(i);
        }
        for i in index..len.min(index + 5) {
            self.update_ao5(i);
        }
        for i in index..len.min(index + 12) {
            self.update_ao12(i);
        }
    }

    pub fn update_statistics_last(&mut self) {
        let index = self.entries.len() - 1;
        self.update_mo3(index);
        self.update_ao5(index);
        self.update_ao12(index);
    }

    fn update_mo3(&mut self, index: usize) {
        let mo3 = self.compute_mo3(index);
        if let Some(entry) = self.entries.get_mut(index) {
            entry.mo3 = mo3;
        }
    }

    fn update_ao5(&mut self, index: usize) {
        let ao5 = self.compute_ao5(index);
        if let Some(entry) = self.entries.get_mut(index) {
            entry.ao5 = ao5;
        }
    }

    fn update_ao12(&mut self, index: usize) {
        let ao12 = self.compute_ao12(index);
        if let Some(entry) = self.entries.get_mut(index) {
            entry.ao12 = ao12;
        }
    }
}
