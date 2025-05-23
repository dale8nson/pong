mod game;

use crate::game::Game;

fn main() {
    let mut game = Game::new();
    game.initialize();
    game.run_loop();
}
