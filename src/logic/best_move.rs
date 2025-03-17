use crate::parsing::{AnField, pieces::Piece, pieces::Slot};
use crate::parsing::points::Point;
use crate::logic::valid_placement::is_valid_placement;

/// Évalue une position pour une pièce donnée
fn evaluate_position(anfield: &AnField, piece: &Piece, x: usize, y: usize) -> (usize, usize, usize) {
    let mut score = 0;
    let mut touch_count = 0;
    let mut vertical_priority = usize::MAX;

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
                    touch_count += 3;
                }

                let vertical_dist = anfield.self_owned.iter()
                    .map(|p| (p.y as isize - grid_y as isize).abs() as usize)
                    .min()
                    .unwrap_or(usize::MAX);
                
                if vertical_dist < vertical_priority {
                    vertical_priority = vertical_dist;
                }
            }
        }
    }

    (score, touch_count, vertical_priority)
}

/// Trouve la meilleure position pour placer une pièce
pub fn find_best_move(anfield: &AnField, piece: &Piece, first_turn: bool) -> (usize, usize) {
    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_touch = 0;
    let mut best_score = 0;
    let mut best_vertical_priority = usize::MAX;
    let mut best_leftmost = usize::MAX;

    for y in 0..anfield.length as usize {
        for x in 0..anfield.width as usize {
            if is_valid_placement(anfield, piece, x, y, first_turn) {
                let (score, touch, vertical_priority) = evaluate_position(anfield, piece, x, y);

                println!(
                    "Testing position ({}, {}): score={}, touch={}, vertical_priority={}",
                    x, y, score, touch, vertical_priority
                );

                if touch > best_touch || 
                   (touch == best_touch && vertical_priority < best_vertical_priority) || 
                   (touch == best_touch && vertical_priority == best_vertical_priority && score > best_score) || 
                   (touch == best_touch && vertical_priority == best_vertical_priority && score == best_score && x < best_leftmost) 
                {
                    best_touch = touch;
                    best_score = score;
                    best_x = x;
                    best_y = y;
                    best_vertical_priority = vertical_priority;
                    best_leftmost = x;
                }
            }
        }
    }

    println!(
        "Best move chosen: ({}, {}) with score={}, touch={}, vertical_priority={}",
        best_x, best_y, best_score, best_touch, best_vertical_priority
    );

    (best_x, best_y)
}
