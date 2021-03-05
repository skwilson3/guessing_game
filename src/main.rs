extern crate guessing_game;

use guessing_game::Game;

fn main(){
    guessing_game::welcome();
    guessing_game::explain_rules();
    guessing_game::explain_difficulties();
    let game = Game::new();
    guessing_game::run(game);
}
