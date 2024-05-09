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

        /*
        Rust allows to re-use the previous variable "guess" with something called Shadowing.
        Shadowing is a technique where a variable is re-assigned with a new value, but the old value is still accessible.
        match is a control flow statement that allows you to compare values and execute different blocks of code based on the comparison.
        Err is an enum that represents an error, and it is used to handle errors in a match expression, the _ is a wildcard that matches any value.
        The continue keyword is used to skip the rest of the current block of code and continue with the next block.
        The `trim()` method is used to remove any leading or trailing whitespace from a string.
        The parse() method is used to convert the string to a u8 data type.
        Ordering::Less, Ordering::Greater, and Ordering::Equal are enums that represent the three possible outcomes of a comparison.
        */
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        println!("You entered: {}", guess);

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
