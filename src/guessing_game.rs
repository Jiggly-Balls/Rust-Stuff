use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Stdin};

pub fn run() {
    let stdin: Stdin = io::stdin();
    let mut tries: u32 = 3;
    let secret_number: u32 = rand::rng().random_range(1..=10);

    loop {
        let mut guess: String = String::new();

        println!("Guess a number: ");

        stdin.read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number\n");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You got it correct!");
                break;
            }
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Less => println!("Too small!\n"),
        };

        tries -= 1;

        println!("You have {tries} tries left!");

        if tries == 0 {
            println!("You lost!");
            println!("The secret number was: {secret_number}");
            break;
        }
    }
}
