extern crate guessing_game;

use guessing_game::Difficulty;
use guessing_game::Game;

fn main(){
    guessing_game::welcome();
    guessing_game::explain_rules();
    guessing_game::explain_difficulties();
    let diff = guessing_game::get_difficulty();
    let game = Game::new(diff);
    guessing_game::run(game);
}
