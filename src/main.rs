use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    /*
    Generate a random number between 1 and 100 using the `rand` crate.
    The two dots ".." denote a range that includes the start value but excludes the end value.
    The equal sign "=" after the two dots indicates that the range is inclusive, meaning it includes both the start and the end values.
    */
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    println!("Enter a number between 1 and 100");

    // a variable to store the user's guess.
    let mut guess = String::new();

    /*
    Read the user's input from the console. If the user enters an invalid input,
    the program will continue to prompt the user for input until a valid input is entered.
    */
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You entered: {}", guess);

    /*
    Rust allows us to re-use the previous value of guess with something called Shadowing
    The `trim()` method is used to remove any leading or trailing whitespace from a string.
    The parse() method is used to convert the string to a u32 data type.
    Ordering::Less, Ordering::Greater, and Ordering::Equal are enums that represent the three possible outcomes of a comparison.
    */
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
