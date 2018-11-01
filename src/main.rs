extern crate rand;

mod event;
mod game;
mod player;
mod team;


fn main() {
    println!("Starting game");
    let mut g = game::Game::new();
    g.finish_game();
    println!("Finised game");
}
