// Imports required libraries
use rand::Rng;
use std::io;

// Main function
fn main() {
    // Print greeting
    println!("Welcome to the guessing game!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        // Prompt user for guess
        println!("Please guess a number between 1 and 100:");

        // Mutable string to hold user input
        let mut guess = String::new();

        // Read user input
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {}
            Err(error) => {
                println!("Error reading input: {}", error);
                continue;
            }
        }

        // Trim newline character from input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        // Check if guess is correct
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
