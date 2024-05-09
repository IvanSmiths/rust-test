use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Guess the number!");

    /*
    Generate a random number between 1 and 100 using the `rand` crate.
    The two dots ".." denote a range that includes the start value but excludes the end value.
    The equal sign "=" after the two dots indicates that the range is inclusive, meaning it includes both the start and the end values.
    */
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    // The loop will continue to run until the user enters a valid number.
    loop {
        println!("Enter a number between 1 and 100");

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
        Rust allows to re-use the previous variable "guess" with something called Shadowing.
        The `trim()` method is used to remove any leading or trailing whitespace from a string.
        The parse() method is used to convert the string to a u8 data type.
        Ordering::Less, Ordering::Greater, and Ordering::Equal are enums that represent the three possible outcomes of a comparison.
        */
        let guess: u8 = guess.trim().parse().expect("Please type a number!");

        println!("Please enter a number between 1 and 100");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                thread::sleep(Duration::from_secs(2));
                break;
            }
        }
    }
}
