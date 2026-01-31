// Import the io (input/output) module from the standard library
// This gives us access to functions for reading user input
use std::io;

// The main function is the entry point of every Rust program
// It runs automatically when you execute the program
fn main() {
    // println! is a macro (indicated by the !) that prints text to the console
    // The text in quotes is a string literal
    println!("Guess the number!");

    println!("Please input your guess.");

    // Create a new variable called 'guess'
    // 'let' declares a variable
    // 'mut' makes it mutable (changeable) - variables are immutable by default in Rust
    // String::new() creates a new, empty String
    // The :: syntax means we're calling an "associated function" on the String type
    let mut guess = String::new();

    // Call the stdin() function from the io module to get a handle to standard input
    io::stdin()
        // read_line() reads input from the user and stores it in the guess variable
        // &mut means we're passing a mutable reference to guess (allows the function to modify it)
        // The & is a reference - it lets the function use the variable without taking ownership
        .read_line(&mut guess)
        // read_line() returns a Result type which could be Ok or Err
        // expect() handles potential errors - if reading fails, it crashes with this message
        .expect("Failed to read line");

    // Print the user's guess
    // {guess} is string interpolation - it inserts the value of guess into the output
    // This is shorthand for {guess: guess} in older Rust versions
    println!("You guessed: {guess}");
}
