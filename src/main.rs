extern crate guessing_game;

use guessing_game::Difficulty;
use guessing_game::Game;

fn main(){
    guessing_game::welcome();
    guessing_game::explain_rules();
    let game = Game::new(Difficulty::Easy);
    guessing_game::run(game);
}
