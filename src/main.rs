use std::io;
use std::io::prelude::*;

fn main(){
    print!("Enter your guess: ");
    io::stdout().flush().unwrap();
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);
}