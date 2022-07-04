use std::{cmp::Ordering, collections::HashSet, simd::{u8x8, ToBitMask}};

use crate::eval_counter::EvaluationCounter;

pub type PinVec = [u8; 8];
pub type ColorVec = [u8; 8];

#[derive(Debug, Clone)]
pub struct StandardCollection {
    size: (usize, usize),
    pins: Vec<PinVec>,
    colors: Vec<ColorVec>,
}

impl StandardCollection {
    pub fn new(capacity: usize, pins: usize, colors: usize) -> Self {
        assert!(pins <= 8);
        assert!(colors <= 8);
        Self {
            size: (pins, colors),
            pins: Vec::with_capacity(capacity),
            colors: Vec::with_capacity(capacity),
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.size
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.pins.len(), self.colors.len());
        self.pins.len()
    }

    pub fn push(&mut self, position: &[u8]) {
        let (pin_count, _) = self.size;
        assert_eq!(position.len(), pin_count);
        let mut color_vec = [0; 8];
        let mut pin_vec = [0; 8];
        for (i, pin) in position.iter().enumerate() {
            color_vec[*pin as usize] += 1;
            pin_vec[i] = *pin;
        }
        self.pins.push(pin_vec);
        self.colors.push(color_vec);
    }

    pub fn generate_possibilities(pins: usize, colors: usize) -> Self {
        let mut collection = Self::new(pins.pow(colors as u32), pins, colors);
        let mut indexes = vec![0; pins];

        loop {
            collection.push(&indexes);

            match indexes.iter().position(|&c| c + 1 < colors as u8){
                Some(position) => {
                    for k in 0..position {
                        indexes[k] = 0
                    }
                    indexes[position] += 1
                }
                None => break
            };
        }
        
        collection
    }

    pub fn prune(&mut self, position: &[u8], black_pins: u8, white_pins: u8) {
        let results = self.evaluate(position);
        let (pin_count, color_count) = self.get_size();
        let mut pruned = Self::new(self.len(), pin_count, color_count);

        self.into_iter()
            .zip(results.into_iter())
            .filter_map(|(p, result)| {
                if *result == (black_pins, white_pins) {
                    Some(p)
                } else {
                    None
                }
            })
            .for_each(|pins| pruned.push(pins));

        *self = pruned
    }

    pub fn find_best_guess<'a>(&self, guesses: &'a Self) -> Option<(&'a [u8], usize)> {
        let worst_cases = self.get_worst_cases(guesses);
        let possibilities_set: HashSet<&[u8]> = self.into_iter().collect();

        let (guess, worst_case) = guesses.into_iter().zip(worst_cases.into_iter()).min_by(
            |(_, max_p1), (p2, max_p2)| {
                match max_p1.cmp(max_p2) {
                    //If both guesses yield the same worst case prioritize lucky guess
                    Ordering::Equal if possibilities_set.contains(p2) => Ordering::Greater,
                    order => order,
                }
            },
        )?;
        Some((guess, *worst_case))
    }

    fn evaluate_single(
        &self,
        pins: PinVec,
        colors: ColorVec,
        other_pins: &PinVec,
        other_colors: &ColorVec,
    ) -> (u8, u8) {

        let pin_count = self.size.0;

        let pin_vec = u8x8::from_array(pins);
        let color_vec = u8x8::from_array(colors);

        let other_pin_vec = u8x8::from_array(*other_pins);
        let other_color_vec = u8x8::from_array(*other_colors);

        let black_pins = pin_vec.lanes_eq(other_pin_vec).to_bitmask().count_ones() as u8 - (8 - pin_count) as u8; 
        let matching_colors = other_color_vec.min(color_vec);
        let white_pins = matching_colors.reduce_sum() - black_pins;

        (black_pins, white_pins)
    }

    pub fn evaluate(&self, position: &[u8]) -> Box<[(u8, u8)]> {
        assert!(position.len() <= 8);
        let (_, color_count) = self.size;
        let mut position_colors = [0; 8];
        let mut position_array = [0; 8];

        for (i, p) in position.into_iter().enumerate() {
            assert!(*p < color_count as u8);
            position_colors[*p as usize] += 1;
            position_array[i] = *p;
        }

        return self
            .pins
            .iter()
            .zip(self.colors.iter())
            .map(|(pins, colors)| {
                self.evaluate_single(position_array, position_colors, pins, colors)
            })
            .collect();
    }

    #[cfg(feature = "multithreading")]
    pub fn get_worst_cases(&self, guesses: &Self) -> Box<[usize]> {
        use rayon::{prelude::*};
        let (pin_count, _) = self.get_size();
        
        (&guesses.pins).into_par_iter().map(|p| {
            let mut counter = EvaluationCounter::<usize>::new(pin_count);
            let results = self.evaluate(&p[..pin_count]);
            for (b, w) in results.into_iter() {
                counter.increment(*b as usize, *w as usize)
            }
            counter.max()
        }).collect::<Vec<_>>().into_boxed_slice()
    }

    #[cfg(not(feature = "multithreading"))]
    pub fn get_worst_cases(&self, guesses: &Self) -> Box<[usize]> {
        let (pin_count, _) = self.get_size();
        
        (&guesses.pins).into_iter().map(|p| {
            let mut counter = EvaluationCounter::<usize>::new(pin_count);
            let results = self.evaluate(&p[..pin_count]);
            for (b, w) in results.into_iter() {
                counter.increment(*b as usize, *w as usize)
            }
            counter.max()
        }).collect::<Vec<_>>().into_boxed_slice()
    }
}

#[test]
fn evaluate_test() {
    let positions = StandardCollection::generate_possibilities(2, 2);
    assert_eq!(positions.into_iter().len(), 4);
    let results = positions.evaluate(&[0, 1]);
    assert_eq!(results.len(), 4);
    assert_eq!(results[0], (1, 0)); // 0 0
    assert_eq!(results[1], (2, 0)); // 0 1
    assert_eq!(results[2], (0, 2)); // 1 0
    assert_eq!(results[3], (1, 0)); // 1 1
}

#[test]
fn iter_test() {
    let positions = StandardCollection::generate_possibilities(3, 4);
    let positions_iter = positions.into_iter();
    assert_eq!(positions_iter.len(), 64);
}

impl<'a> IntoIterator for &'a StandardCollection {
    type Item = &'a [u8];
    type IntoIter = impl ExactSizeIterator<Item = Self::Item> + 'a;

    fn into_iter(self) -> Self::IntoIter {
        let (pin_count, _) = self.size;
        self.pins.iter().map(move |p| &p[..pin_count])
    }
}