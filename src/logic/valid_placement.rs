use crate::parsing::{AnField, Piece, Slot};  
use crate::parsing::points::Point;

pub fn is_valid_placement(anfield: &AnField, piece: &Piece, x: usize, y: usize) -> bool {
    let mut touch_count = 0;

    for slot in &piece.slots {
        if let Slot::Used(slot_point) = slot {
            let grid_x = x as u32 + slot_point.x;
            let grid_y = y as u32 + slot_point.y;

            // verifi que la pièce reste dans les limites de l'Anfield
            if grid_x >= anfield.width || grid_y >= anfield.length {
                return false;
            }

            let point = Point::new(grid_x, grid_y);

            //  verifie qu’on ne recouvre pas une cellule adverse
            if anfield.ennemie_owned.contains(&point) {
                return false;
            }

            // verifi si cette cellule touche une cellule de ton territoire
            if anfield.self_owned.contains(&point) {
                touch_count += 1;
            }
        }
    }

    //  pièce est valide uniquement si elle touche exactement une cellule de ton territoire
    touch_count == 1
}
