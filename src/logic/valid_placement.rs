use crate::parsing::{AnField, pieces::Piece, pieces::Slot};  
use crate::parsing::points::Point;

pub fn is_valid_placement(anfield: &AnField, piece: &Piece, x: usize, y: usize, first_turn: bool) -> bool {
    let mut touch_count = 0;

    for slot in &piece.slots {
        if let Slot::Used(slot_point) = slot {
            let grid_x = x as u32 + slot_point.x;
            let grid_y = y as u32 + slot_point.y;

            // verifie que la pièce reste dans les limites de l'Anfield
            if grid_x >= anfield.width || grid_y >= anfield.length {
                return false;
            }

            let point = Point::new(grid_x, grid_y);

            // verifie qu’on ne recouvre pas une cellule adverse
            if anfield.ennemie_owned.contains(&point) {
                return false;
            }

            // verifie si cette cellule touche une cellule de ton territoire
            if anfield.self_owned.contains(&point) {
                touch_count += 1;
            }
        }
    }

    // Si c'est le premier tour, on autorise la pièce même sans contact.
    if first_turn {
        return true;
    }

    // Sinon, la pièce est valide uniquement si elle touche **au moins une** cellule alliée
    touch_count >= 1
}


