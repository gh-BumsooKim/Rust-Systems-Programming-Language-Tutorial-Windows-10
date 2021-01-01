extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Hello, Crate!");

    let num = rand::thread_rng().gen_range(1, 101); // num = 1 ~ 100
    println!("Random Number : {}", num);
}
