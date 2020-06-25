// This program will generate a random integer between 1 and 100.
// It will then prompt the player to enter a guess.
// After a guess is entered, the program will indicate whether the guess was too high or low.
// If the guess is correct, a congratulatory message will be printed and the game will exit.

use std::io;
use std::cmp::Ordering;
use rand::{thread_rng, Rng};

fn main() {
    let secret_number = thread_rng().gen_range(1, 101);

    println!("Guess the number!");

    loop {
        let mut guess = String::new();

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Trim the \n resulting from user pressing enter, then parse into integer
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number! Try again!\n");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        }
    }
}
