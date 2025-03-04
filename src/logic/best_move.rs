use crate::parsing::{AnField, Piece};
use crate::parsing::points::Point;
use crate::parsing::pieces::Slot;
use crate::logic::valid_placement::is_valid_placement;

// la meilleure position pour placer la pièce et retourne (x, y)
pub fn find_best_move(anfield: &AnField, piece: &Piece) -> (usize, usize) {
    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_score = 0;

    for y in 0..anfield.length {
        for x in 0..anfield.width {
            if is_valid_placement(anfield, piece, x as usize, y as usize) {
                let score = evaluate_position(anfield, piece, x as usize, y as usize);

                if score > best_score {
                    best_score = score;
                    best_x = x as usize;
                    best_y = y as usize;
                }
            }
        }
    }

    (best_x, best_y)
}

// calcul score de placement 
fn evaluate_position(anfield: &AnField, piece: &Piece, x: usize, y: usize) -> usize {
    let mut score = 0;

    for slot in &piece.slots {
        if let Slot::Used(piece_point) = slot {
            let grid_x = x as u32 + piece_point.x;
            let grid_y = y as u32 + piece_point.y;

            if grid_x < anfield.width && grid_y < anfield.length {
                let point = Point::new(grid_x, grid_y);
                if !anfield.ennemie_owned.contains(&point) && !anfield.self_owned.contains(&point) {
                    score += 1;
                }
            }
        }
    }

    score
}

// toutes les coordonnées globales occupées par la pièce lorsqu'elle est placée
// pub fn get_piece_global_coordinates(x: usize, y: usize, piece: &Piece) -> Vec<(u32, u32)> {
//     let mut coordinates = Vec::new();

//     for slot in &piece.slots {
//         if let Slot::Used(slot_point) = slot {
//             let global_x = x as u32 + slot_point.x;
//             let global_y = y as u32 + slot_point.y;
//             coordinates.push((global_x, global_y));
//         }
//     }

//     coordinates

// }

pub fn get_piece_global_coordinates(x: usize, y: usize, _piece: &Piece) -> (u32, u32) {
    (x as u32, y as u32)
}
