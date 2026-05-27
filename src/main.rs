use std::io;
use rand::{RngExt};

fn main() {
    println!("Please input your guess number");

    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is : {}", secret_number);

    let mut guess = String::new();

    io::stdin().
        read_line(&mut guess).
        expect("Failed to read line");

    println!("You guessed : {}", guess);

}
