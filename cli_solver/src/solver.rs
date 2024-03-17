use dialoguer::Input;

use num_format::{Locale, ToFormattedString};

use std::time::SystemTime;

use libmastermind::positions::StandardCollection;

const P: usize = 7;
const C: usize = 8;

fn main() {
    let mut possibilites = StandardCollection::generate_possibilities(P, C);
    let all_guesses = possibilites.clone();
    let mut attempts = 0;

    let mut previous_guesses = Vec::new();
    

    loop {

        let guesses_before_prune = all_guesses.len();

        let start = SystemTime::now();

        let pruned_guesses =
            possibilites.prune_symmetrys(previous_guesses.as_slice(), &all_guesses);

        println!(
            "Pruned possible guesses from length {} to length {}: -{}%",
            guesses_before_prune,
            pruned_guesses.len(),
            100. - (pruned_guesses.len() as f32 / guesses_before_prune as f32) * 100.
        );
        println!(
            "Need to evaluate {} Positions",
            (pruned_guesses.len() * possibilites.len()).to_formatted_string(&Locale::en)
        );

        let (pins, min_max) = possibilites
            .find_best_guess(&pruned_guesses)
            .expect("No guess found");
        let guess = pins.to_vec();
        println!("Finished after {}ms", start.elapsed().unwrap().as_millis());
        println!(
            "MinMax at depth {}: {:?} @ {:?} / Total Possibilitys reamining: {}",
            attempts, guess, min_max, possibilites.len()
        );
        
        attempts += 1;
        println!("Please enter {:?}", guess);
        let input = Input::<String>::new()
            .with_prompt("B / W")
            .interact()
            .unwrap();
        let nums: Vec<u8> = input.split(' ').map(|x| x.parse::<u8>().unwrap()).collect();
        let (b, a) = (nums[0], nums[1]);

        let possibilities_before_prune = possibilites.len();
        possibilites.prune(&guess, b, a);
        previous_guesses.push(guess);

        let possibilites_after_prune = possibilites.len();
        
        println!(
            "Pruned possible solutions from length {} to length {}: -{}%",
            possibilities_before_prune,
            possibilites_after_prune,
            100. - (possibilites_after_prune as f32 / possibilities_before_prune as f32) * 100.
        );

        if possibilites_after_prune == 1 {
            break
        }
       
    }
    println!(
        "{:?} found after {:?} Attempts",
        &possibilites.into_iter().next().unwrap(),
        attempts
    );
}
