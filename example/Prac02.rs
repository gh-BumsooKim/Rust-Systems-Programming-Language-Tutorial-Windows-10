use std::io;

fn main() {
    println!("Hello, Rust!");
    println!("Input String!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Can't Read Input value");

    println!("Input value : {}", guess);
}