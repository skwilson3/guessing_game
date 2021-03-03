use std::io;
use std::io::prelude::*;

use std::cmp::Ordering;

use rand::Rng;

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct Game {
    pub secret_number: u32,
    pub guesses_left: u32,
}

impl Game{
    pub fn new(diff: Difficulty) -> Game {
        Game{ secret_number: gen_secret_number(diff), guesses_left: 10 }
    }
}

fn gen_secret_number(diff: Difficulty) -> u32 {
    let max_num = match diff {
        Difficulty::Easy => 10,
        Difficulty::Medium => 50,
        Difficulty::Hard => 100,
    };
    rand::thread_rng().gen_range(1..=max_num)
}

fn get_input(prompt: &str) -> Result<String, io::Error>{
    print!("{}",prompt);
    io::stdout().flush().unwrap();
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)?;
    Ok(guess)
}

pub fn welcome() {
    println!(r#"      welcome to...                         "#);
    println!(r#"  ______                     _              "#);
    println!(r#" |  ____|_   _  ____ ___ ___(_)_ __   __ _  "#);
    println!(r#" | |  __| | | |/ __ / __/ __| | '_ \ / _' | "#);
    println!(r#" | |_|_ | |_| |  ___\__ \__ | | | | | (_| | "#);
    println!(r#" |______|\__,_|\____|___/___|_|_| |_|\__, | "#);
    println!(r#"  ______                             |___/  "#);
    println!(r#" |  ____| __ _ _ __ ___   ____              "#);
    println!(r#" | |  __|/ _' | '_ Y _ \ / __ \             "#);
    println!(r#" | |_|_ | (_| | | | | | |  ___/             "#);
    println!(r#" |______|\__,_|_| |_| |_|\____|             "#);
    println!(r#"                                            "#);
}

pub fn explain_rules() {
    println!("The object of this game is to guess the secret");
    println!("number between 1 and 100. The computer will tell");
    println!("if your guess is too high or too low and then");
    println!("you can guess again!");
    println!("");
}

pub fn run(game: Game) {

    for ctr in 0..game.guesses_left {

        println!("Guesses left: {}", game.guesses_left-ctr);

        let guess = match get_input("Enter your guess: ") {
            Ok(s) => {
                if s.to_lowercase()=="quit" {
                    break;
                } else {
                    s
                }
            },
            Err(e) => {
                println!("{:#}", e);
                continue;
            },
        };

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                println!("{:#}", e);
                continue;
            },
        };

        match guess.cmp(&game.secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }

    println!("Game Over!");
}