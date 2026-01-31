use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Type 'quit' to exit.");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        if guess == "quit" {
            println!("Goodbye!");
            break;
        }

        // Try to convert the string to a u32 number
        // parse() returns a Result type which can be either Ok(value) or Err(error)
        let guess: u32 = match guess.parse() {
            // If parsing succeeds, Ok(num) contains the successfully parsed number
            // We extract and use that number
            Ok(num) => num,
            // If parsing fails (e.g., user typed "abc" or "12.5"), Err contains the error
            // The underscore _ means we don't care about the specific error details
            Err(_) => {
                println!("Please type a number or 'quit'!");
                // 'continue' skips the rest of this loop iteration and starts a new one
                // This prevents the program from crashing on invalid input
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
