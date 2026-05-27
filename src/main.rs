use std::io;
use rand::{RngExt};
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess number");

        // println!("The secret number is : {}", secret_number);

        let mut guess = String::new();

        io::stdin().
            read_line(&mut guess).
            expect("Failed to read line");

        // let guess = guess.trim().parse::<u32>().expect("Type a freakin number dumbass!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("This is not a freakin number dumbass!");
                continue 
            },
        };


        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Your guessed is right");
                println!("You guessed : {}", guess);
                break;
            },
            Ordering::Greater => {
                println!("Your guessed number too big");
                println!("You guessed : {}", guess);
                println!("Guessed again!");
            }
            Ordering::Less => {
                println!("Your guessed number is lessed than what it generated");
                println!("You guessed : {}", guess);
                println!("Guessed again!");
            }
        }   
    }
}
