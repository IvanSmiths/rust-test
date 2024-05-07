use std::io;

fn main() {
    println!("Guess the number!");
    println!("Enter a number between 1 and 100");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You entered: {}", guess);
}
