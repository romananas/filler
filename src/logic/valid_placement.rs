use crate::parsing::{AnField, pieces::Piece, pieces::Slot};  
use crate::parsing::points::Point;

/// Vérifie si un placement est valide
pub fn is_valid_placement(anfield: &AnField, piece: &Piece, x: usize, y: usize, first_turn: bool) -> bool {
    let mut touch_count = 0;
    let mut occupied_by_enemy = false;

    for slot in &piece.slots {
        if let Slot::Used(slot_point) = slot {
            let grid_x = x as u32 + slot_point.x;
            let grid_y = y as u32 + slot_point.y;

            if grid_x >= anfield.width || grid_y >= anfield.length {
                return false;
            }

            let point = Point::new(grid_x, grid_y);

            if anfield.ennemie_owned.contains(&point) {
                occupied_by_enemy = true;
            }

            if anfield.self_owned.contains(&point) {
                touch_count += 2; // ✅ Donne plus de poids au contact avec un allié
            }
        }
    }

    if occupied_by_enemy {
        return false;
    }

    if first_turn {
        return true;
    }

    touch_count >= 2 // ✅ Doit avoir au moins un vrai contact
}