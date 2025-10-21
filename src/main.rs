use std::io::{self, Stdin};

fn main() {
    let mut guess: String = String::new();
    let stdin: Stdin = io::stdin();

    println!("Guess a number: ");

    stdin.read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {guess}");
}
