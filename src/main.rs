mod parsing;
mod logic;
mod display;

use logic::read_input::read_input;
use logic::best_move::find_best_move;
use logic::print_move::print_move;

fn main() {
    // Lire l'état du jeu via read_input
    let game_state = read_input();

    // Trouver la meilleure position
    let (x, y) = find_best_move(&game_state.anfield, &game_state.anfield.piece);

    // Afficher la position choisie
    print_move(x, y);
}

/// Fonction pour lire l'entrée complète de stdin
pub fn get_stdin() -> String {
    let mut complete_input = String::new();
    let mut end_loop = (false, 0);

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        complete_input += &input;

        if input.starts_with("Piece") {
            let tmp: Vec<&str> = input.split_whitespace().collect();
            let mut t = tmp[tmp.len() - 1];
            t = t.trim_matches(':');

            end_loop.0 = true;
            end_loop.1 = t.parse().unwrap();

            continue;
        }

        if end_loop.0 {
            end_loop.1 -= 1;

            if end_loop.1 == 0 {
                break;
            }
        }
    }

    complete_input
}
