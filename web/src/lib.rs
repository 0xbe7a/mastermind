#[cfg(target_feature = "atomics")]
pub use wasm_bindgen_rayon::init_thread_pool;
use libmastermind::positions::StandardCollection;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solver {
    possibilites: StandardCollection,
    all_guesses: StandardCollection,
    previous_guesses: Vec<Vec<u8>>,
}

#[wasm_bindgen]
pub struct Guess {
    guess: Vec<u8>,
    worst_case: usize,
    possibilities: usize,
    pre_prune_guesses: usize,
    post_prune_guesses: usize,
}

#[wasm_bindgen]
impl Guess {
    #[wasm_bindgen]
    pub fn get_guess(&self) -> Vec<u8> {
        self.guess.clone()
    }
    #[wasm_bindgen]
    pub fn get_worst_case(&self) -> usize {
        self.worst_case
    }
    #[wasm_bindgen]
    pub fn get_possibilities(&self) -> usize {
        self.possibilities
    }
    #[wasm_bindgen]
    pub fn get_pre_prune_guesses(&self) -> usize {
        self.pre_prune_guesses
    }
    #[wasm_bindgen]
    pub fn get_post_prune_guesses(&self) -> usize {
        self.post_prune_guesses
    }
}



#[wasm_bindgen]
impl Solver {
    #[wasm_bindgen]
    pub fn init(pins: usize, colors: usize) -> Self {
        let possibilites = StandardCollection::generate_possibilities(pins, colors);
        let all_guesses = possibilites.clone();
        let previous_guesses = Vec::new();
        Self {
            possibilites,
            all_guesses,
            previous_guesses,
        }
    }

    #[wasm_bindgen]
    pub fn min_max(&mut self) -> Guess {
        let guesses_before_prune = self.all_guesses.len();

        let pruned_guesses = self
            .possibilites
            .prune_symmetrys(self.previous_guesses.as_slice(), &self.all_guesses);

        let (pins, worst_case) = self
            .possibilites
            .find_best_guess(&pruned_guesses)
            .expect("No guess found");

        Guess {
            guess: pins.to_vec(),
            worst_case,
            possibilities: self.possibilites.len(),
            pre_prune_guesses: guesses_before_prune,
            post_prune_guesses: pruned_guesses.len(),
        }
    }

    #[wasm_bindgen]
    pub fn add_guess_result(&mut self, guess: &[u8], black_pins: u8, white_pins: u8) -> usize {
        self.previous_guesses.push(guess.to_vec());
        self.possibilites.prune(guess, black_pins, white_pins);
        self.possibilites.len()
    }

    #[wasm_bindgen]
    pub fn reset_guesses(&mut self) {
        self.possibilites = self.all_guesses.clone();
        self.previous_guesses.clear();
    }

    #[wasm_bindgen]
    pub fn get_final_awnser(&self) -> Option<Vec<u8>> {
        self.possibilites.into_iter().next().map(|p| p.to_vec())
    }
}
