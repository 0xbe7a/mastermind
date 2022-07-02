use num::{Zero, One};

pub struct EvaluationCounter<T> {
    evaluations: Vec<T>,
    pins: usize,
}

/*
    Example: Memory Layout for 3 Pins

      | 0   1   2   3  -> Black Pins
    --|-------------------------------
    3 | 0
    2 | 1   2
    1 | 3   4   5
    0 | 6   7   8   9
    |
    v
  White Pins

  (black, white) -> linear: ((N - white) * (N - white + 1) / 2) + black

*/

impl<T> EvaluationCounter<T> where T: Zero + One + Clone + Ord {
    pub fn new(pins: usize) -> Self {
        let total_pins = ((pins + 1) * (pins + 2)) / 2;
        Self {
            evaluations: vec![T::zero(); total_pins],
            pins
        }
    }

    pub fn increment(&mut self, black: usize, white: usize) {
        let k = ((self.pins - white) * (self.pins - white + 1) / 2) + black;
        self.evaluations[k] = T::one().add(self.evaluations[k].clone());
    }

    pub fn max(&self) -> T {
        self.evaluations.iter().max().unwrap().clone()
    }
}

