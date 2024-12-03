use rand::Rng;
use std::{cmp::Ordering, io};

/// Generates a secret number between 1 and 100
fn generate_secret_number() -> u8 {
    rand::thread_rng().gen_range(1..=100)
}

/// Reads and parses user input, returns a `Result`  allowing errors to be handled explicitly
fn get_user_guess() -> Result<u8, &'static str> {
    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .map_err(|_| "Failed to read line.")
        .expect("Failed to read line.");

    guess.trim().parse().map_err(|_| "Invalid number.")
}

/// Compare the guess with the secret number and display the result
fn compare_guess(guess: u8, secret_number: u8) -> Ordering {
    guess.cmp(&secret_number)
}

/// Main function of the game
fn play_game() {
    println!("Guess the number!");

    let secret_number: u8 = generate_secret_number();

    loop {
        println!("Please input your guess.");

        let guess: u8 = match get_user_guess() {
            Ok(num) => num,
            Err(error) => {
                println!("Error: {error}");
                continue;
            }
        };

        match compare_guess(guess, secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}

fn main() {
    play_game()
}
