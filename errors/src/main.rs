use guessing_game::guessing_game;
use result::read_username_from_file;

mod guessing_game;
mod result;

fn main() {
    println!("Hello, world!");

    // read_username_from_file();
    guessing_game()
}
