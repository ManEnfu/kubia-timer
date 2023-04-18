use super::*;

#[derive(Clone)]
pub struct Session {
    solves: Vec<Solve>,
    mo3: Vec<SolveTime>,
    ao5: Vec<SolveTime>,
    ao12: Vec<SolveTime>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            solves: Vec::new(),
            mo3: Vec::new(),
            ao5: Vec::new(),
            ao12: Vec::new(),
        }
    }

    pub fn add_solve(&mut self, solve: Solve) {
        let len = self.solves.len();
        self.solves.push(solve);

        if let Some(mo3) = self.compute_mo3(len) {
            self.mo3.push(mo3);
        }
        if let Some(ao5) = self.compute_ao5(len) {
            self.ao5.push(ao5);
        }
        if let Some(ao12) = self.compute_ao12(len) {
            self.ao12.push(ao12);
        }
    }

    pub fn get_n_solves(&self) -> usize {
        self.solves.len()
    }

    pub fn get_solve(&self, index: usize) -> Option<&Solve> {
        self.solves.get(index)
    }

    pub fn get_mo3(&self, index: usize) -> Option<SolveTime> {
        if self.solves.len() >= 3 {
            self.mo3.get(index - 3).copied()
        } else {
            None
        }
    }

    pub fn get_ao5(&self, index: usize) -> Option<SolveTime> {
        if self.solves.len() >= 5 {
            self.ao5.get(index - 5).copied()
        } else {
            None
        }
    }

    pub fn get_ao12(&self, index: usize) -> Option<SolveTime> {
        if self.solves.len() >= 12 {
            self.ao12.get(index - 12).copied()
        } else {
            None
        }
    }

    pub fn best_mo3(&self) -> Option<SolveTime> {
        self.mo3.iter().min().copied()
    }

    pub fn best_ao5(&self) -> Option<SolveTime> {
        self.ao5.iter().min().copied()
    }

    pub fn best_ao12(&self) -> Option<SolveTime> {
        self.ao12.iter().min().copied()
    }

    pub fn last_mo3(&self) -> Option<SolveTime> {
        self.mo3.last().copied()
    }

    pub fn last_ao5(&self) -> Option<SolveTime> {
        self.ao5.last().copied()
    }

    pub fn last_ao12(&self) -> Option<SolveTime> {
        self.ao12.last().copied()
    }

    fn compute_mo3(&mut self, index: usize) -> Option<SolveTime> {
        if self.solves.len() >= 3 {
            self.solves
                .get(index - 2..index + 1)
                .and_then(|solves| solves.mean_of_n())
        } else {
            None
        }
    }

    fn compute_ao5(&mut self, index: usize) -> Option<SolveTime> {
        if self.solves.len() >= 5 {
            self.solves
                .get(index - 4..index + 1)
                .and_then(|solves| solves.average_of_n())
        } else {
            None
        }
    }

    fn compute_ao12(&mut self, index: usize) -> Option<SolveTime> {
        if self.solves.len() >= 12 {
            self.solves
                .get(index - 11..index + 1)
                .and_then(|solves| solves.average_of_n())
        } else {
            None
        }
    }

    fn update_statistics(&mut self, index: usize) {
        self.update_mo3(index);
        self.update_ao5(index);
        self.update_ao12(index);
    }

    fn update_mo3(&mut self, index: usize) {
        if self.solves.len() >= 3 {
            if let Some(new_mo3) = self
                .solves
                .get(index..index + 3)
                .and_then(|solves| solves.mean_of_n())
            {
                self.mo3[index - 3] = new_mo3;
            }
        }
    }

    fn update_ao5(&mut self, index: usize) {
        if self.solves.len() >= 5 {
            if let Some(new_ao5) = self
                .solves
                .get(index..index + 5)
                .and_then(|solves| solves.average_of_n())
            {
                self.ao5[index - 5] = new_ao5;
            }
        }
    }

    fn update_ao12(&mut self, index: usize) {
        if self.solves.len() >= 12 {
            if let Some(new_ao12) = self
                .solves
                .get(index..index + 12)
                .and_then(|solves| solves.average_of_n())
            {
                self.ao12[index - 12] = new_ao12;
            }
        }
    }
}
