// Quasi Reversi
// Black goes first

use manager::{Action, GameManager, Putable};

mod board;
mod manager;

fn main() {
    GameManager::new()
        .game_loop(temp_input, temp_input)
        .show_result();
}

fn temp_input(_board: &Putable) -> Action {
    println!("Please enter x and y separated by a space or 'undo':");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        Action::Put((0, 0))
    } else if parts[0] == "undo" {
        Action::Undo
    } else {
        let x = parts[0].parse().expect("Invalid input for x");
        let y = parts[1].parse().expect("Invalid input for y");
        Action::Put((x, y))
    }
}
