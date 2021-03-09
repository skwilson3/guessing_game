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
                Ok(s) => return String::from(s.trim()),
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

            if allowed_inputs.contains(&&input.to_lowercase()[..]) {
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

    pub fn input_and_validate_fn(prompt: &str, is_allowed: fn(&str) -> bool, default: Option<String>) -> String {
        loop {
            let input = get_input(prompt);

            if is_allowed(&input) {
                return input
            } else if let Some(s) = default {
                return s
            } else {
                println!("Invalid input.");
            }
        }
    }
}

pub mod game{

    use rand::Rng;
    use std::cmp::Ordering;
    
    use crate::utils;

    #[derive(Copy, Clone)]
    pub enum Difficulty {
        Easy,
        Medium,
        Hard,
    }

    impl Difficulty {
        pub fn max_num(self) -> u32 {
            match self {
                Difficulty::Easy => 10,
                Difficulty::Medium => 50,
                Difficulty::Hard => 100,
            }
        }

        pub fn gen_secret_number(self) -> u32 {
            rand::thread_rng().gen_range(1..=self.max_num())
        }
    }

    pub struct Game {
        pub difficulty: Difficulty,
        pub secret_number: u32,
        pub guesses_left: u32,
    }

    impl Game{
        pub fn new() -> Game {
            let diff = get_difficulty();
            Game{ difficulty: diff, secret_number: diff.gen_secret_number(), guesses_left: 10,  }
        }
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

        let num = input.parse().unwrap();

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

    fn allowed_guess(guess: &str) -> bool {
        match guess.chars().next() {
            Some(c) => return c.is_numeric(),
            None => return false,
        }
    }

    pub fn run(game: Game) -> Option<()> {
        for ctr in 0..game.guesses_left {

            println!("Guesses left: {}", game.guesses_left-ctr);

            let guess = utils::input_and_validate_fn("Enter your guess: ", |x| (allowed_guess(x) | (x.trim()=="quit")), None);

            if guess == String::from("quit") {
                return None
            }

            let guess: u32 = guess.trim().parse().unwrap();

            match guess.cmp(&game.secret_number) {
                Ordering::Less => println!("Too small!\n"),
                Ordering::Greater => println!("Too big!\n"),
                Ordering::Equal => {
                    println!("You win!");
                    return Some(())
                }
            }
        }

        println!("Game Over!");
        return Some(())
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
            if let None = game::run(current_game) {
                break
            }

            again = play_again();
        }
        println!("Thanks for playing!");
    }

    fn play_again() -> bool {
        let allowed_inputs = vec!["y","n","yes","no"];
        let input: Vec<char> = utils::input_and_validate("Play again? (Y/n): ", allowed_inputs, None).chars().collect();
        return input[0] == 'y'
    }
}