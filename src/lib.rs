pub mod utils {
    use std::io;
    use std::io::prelude::*;

    pub fn _get_input(prompt: &str) -> Result<String, io::Error>{
        print!("{}",prompt);
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)?;
        Ok(guess)
    }

    pub fn get_input(prompt: &str) -> String {
        loop {
            match _get_input(prompt) {
                Ok(s) => return s,
                Err(e) => {
                    println!("{:#}",e);
                    continue
                }
            }
        }
    }

    pub fn input_and_validate(prompt: &str, allowed_inputs: Vec<&str>, default: Option<String>) -> String {
        loop {
            let input = get_input(prompt);

            if allowed_inputs.contains(&input.trim()) {
                return String::from(input)
            } else {
                if let Some(s) = default {
                    return s
                } else {
                    println!("Invalid input.");
                }
            }
        };
    }

    /*
    pub fn input_and_validate_fn(prompt: &str, is_allowed: fn(&str) -> bool, default: Option<String>) -> String {
        loop {
            let input = get_input(prompt);

            if is_allowed(&input) {
                return input
            } else {
                if let Some(s) = default {
                    return s
                } else {
                    println!("Invalid input.");
                }
            }
        }
    }
    */
}


pub mod game{

    use std::cmp::Ordering;
    use rand::Rng;

    use crate::utils::get_input;

    use crate::utils;

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
        pub fn new() -> Game {
            let diff = get_difficulty();
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
        println!("number. The number will be within some range.");
        println!("The harder the game, the larger the range.");
        println!("The computer will tell if your guess is too");
        println!("high or too low and then you can guess again!\n");
    }

    pub fn explain_difficulties() {
        println!("The following difficulties are availaible.");
        println!("\t1. Easy   -- 1-10");
        println!("\t2. Medium -- 1-50");
        println!("\t3. Hard   -- 1-100");
    }

    fn get_difficulty() -> Difficulty{
        
        let allowed_inputs= vec!["1","2","3"];
        let input = utils::input_and_validate("Please enter difficulty (1-3) [default: Easy]: ", allowed_inputs, Some(String::from("1")));

        let num = input.trim().parse().unwrap();

        match num {
            2 => {
                println!("Medium difficulty selected.\n");
                return Difficulty::Medium
            },
            3 => {
                println!("Hard difficulty selected.\n");
                return Difficulty::Hard
            },
            _ => {
                println!("Easy difficulty selected.\n");
                return Difficulty::Easy
            },
        }
    }

    pub fn run(game: Game) {
        let mut ctr = 0;
        while ctr < game.guesses_left {

            println!("Guesses left: {}", game.guesses_left-ctr);

            let guess = get_input("Enter your guess: ");

            if guess == String::from("quit") {
                break
            }

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
            ctr+=1;
        }

        println!("Game Over!");
    }
}

pub mod instance {
    use crate::game;
    use crate::utils;
    
    pub fn start() {
        game::welcome();
        game::explain_rules();
        game::explain_difficulties();
        let mut current_game: game::Game;
        let mut again = true;
        while again {
            // Play the game
            current_game = game::Game::new();
            game::run(current_game);

            again = play_again();
        }
        println!("Thanks for playing!");
    }

    fn play_again() -> bool {
        let allowed_inputs = vec!["y","Y","n","N"];
        let input: Vec<char> = utils::input_and_validate("Play again? (Y/n): ", allowed_inputs, None).chars().collect();
        return input[0] == 'y'
    }
}