extern crate rand;

use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Number Guessing Game!");

    // Generate a random number between 1 and 150
    let number = rand::thread_rng().gen_range(1, 150);

    // Game loop
    loop {
        println!("Enter a Guess: ");

        // Get the user's guess
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess)
            .expect("Failed to read guess");

        // Parse the guess to a 32-bit unsigned integer
        // Validate the input
        let guess: u32 = match guess.trim().parse() {
            Ok(output) => output,
            Err(_) => continue,
        };

        println!("You guessed {}...", guess);

        // Determine if the guess was less, equal, or greater than the secret number
        match guess.cmp(&number) {
            Ordering::Less => println!("Your guess is too small!"),
            Ordering::Equal => {
                println!("Your guess is correct! You win!");
                break;
            },
            Ordering::Greater => println!("Your guess is too large!"),
        }
    }
}
