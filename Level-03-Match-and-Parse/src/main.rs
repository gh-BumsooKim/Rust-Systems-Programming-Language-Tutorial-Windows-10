extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Hello, Match!");

    let random_num = rand::thread_rng().gen_range(1, 101);
    println!("Random number was generated, Guess this number.");

    println!("Input your number.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // guess - String type -> u32 type
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    // match num == guess
    match random_num.cmp(&guess) {
        Ordering::Less => println!("Too Big."),
        Ordering::Greater => println!("Too Small."),
        Ordering::Equal => println!("You Win!"),
    }
}