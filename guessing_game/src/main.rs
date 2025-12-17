use std::cmp::Ordering;
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

    /* This line creates a new variable with the name guess. Rust lets you reuse
       variable names meaning you can change the type of a variable after its declared.
       u32 is an unsigned 32 bit int. Using the : explicitly makes guess of type u32
       .trim removes while space at the start and end of a string similar to javas .trim for strings
       .parse parses the string into the desired variable type, which is u32 in this case*/
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // Print the guess that the user entered.
    println!("You guessed: {guess}");

    /* Match is a function that uses an comparison enum to handle all the comparison
       posibilities, such as >, <, or =.
       .cmp is a method used to compare two variables like .equals in java.
       Ordering::Less, Ordering::Greater and Ordering::equal represent the
       different comparison results, and the => is used to handle what
       should be done when the condition is met. */
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
