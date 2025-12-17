use std::io;// Standard IO library
use rand::Rng;// Random library imported from rand crate
fn main() {
    println!("Guess the number!");

    /* thread_rng is a specific random number generation function that generates a random
       number in the existing program thread. The number is seeded automatically by the OS
       .get_range() is a method used to specify the range of the random numbers.
       The range notation is start..=end with start and end being inclusive.*/
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    /* let - Used to create a variable
     * mut - Used to make the variable mutable. Variables are immutable
     * by default.
     * String::new() - Created a new instance of a String that is empty.*/
    let mut guess = String::new();

    /* stdin - a function used to handle user input
     * read_line is a funciton used to read a string as input from the user.
     * & is used to pass in a reference
     * nut has to be included to make the reference mutable, since references
     * immutable by default.
     * .expect() prints an error message if the Result enum is equal to Err.
     * This is not the proper way to handle errors. */
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Print the guess that the user entered.
    println!("You guessed: {guess}");
}
