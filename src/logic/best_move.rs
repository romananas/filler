use crate::parsing::{AnField, pieces::Piece, pieces::Slot};
use crate::parsing::points::Point;
use crate::logic::valid_placement::is_valid_placement;

// la meilleure position pour placer la pièce et retourne (x, y)
pub fn find_best_move(anfield: &AnField, piece: &Piece, first_turn: bool) -> (usize, usize) {
    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_score = 0;
    let mut best_touch = 0;

    for y in 0..anfield.length {
        for x in 0..anfield.width {
            if is_valid_placement(anfield, piece, x as usize, y as usize, first_turn) {
                let (score, touch) = evaluate_position(anfield, piece, x as usize, y as usize);

                // priorité aux positions qui touchent des cellules alliées
                if touch > best_touch || (touch == best_touch && score > best_score) {
                    best_touch = touch;
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
fn evaluate_position(anfield: &AnField, piece: &Piece, x: usize, y: usize) -> (usize, usize) {
    let mut score = 0;
    let mut touch_count = 0;

    for slot in &piece.slots {
        if let Slot::Used(piece_point) = slot {
            let grid_x = x as u32 + piece_point.x;
            let grid_y = y as u32 + piece_point.y;

            if grid_x < anfield.width && grid_y < anfield.length {
                let point = Point::new(grid_x, grid_y);
                
                if !anfield.ennemie_owned.contains(&point) && !anfield.self_owned.contains(&point) {
                    score += 1;
                }
                
                if anfield.self_owned.contains(&point) {
                    touch_count += 1;
                }
            }
        }
    }

    (score, touch_count)
}


/*
    
*/