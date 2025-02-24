
use super::points::Point;

#[derive(Debug,Clone, Copy,PartialEq, Eq)]
pub enum Slot {
    Used(Point),
    Unused(Point),
    Unknown(Point),
}

#[derive(Debug,Clone,PartialEq, Eq)]
pub struct Piece {
    pub slots: Vec<Slot>,
}

impl Piece {
    pub fn from_str<'a>(map: &'a str, used: &'a Option<Vec<char>>, unused: char) -> Result<Self, &'a str> {
        match Piece::verify_format(used.clone(), unused) {
            Some(c) => return Err(Box::leak(format!("can't have the same char in used and unused, found : {}", c).into_boxed_str())),
            None => {},
        }
        let mut slots: Vec<Slot> = Vec::new();
        let splited_map = map.split("\n").collect::<Vec<&str>>();
        for (y,line) in splited_map.iter().enumerate() {
            for (x,c_char) in line.chars().enumerate() {
                if c_char == unused {
                    slots.push(Slot::Unused(Point::new(x as u32, y as u32)));
                    continue;
                }
                match &used {
                    Some(ref chars) => {
                        if chars.contains(&c_char) {
                            slots.push(Slot::Used(Point::new( x as u32, y as u32)));
                        } else {
                            slots.push(Slot::Unknown(Point::new( x as u32, y as u32)));
                        }
                    }
                    None => slots.push(Slot::Used(Point::new(x as u32, y as u32))),
                }
            }
        }
        return Ok(Self {
            slots
        });
    }

    pub fn from_vec(map: Vec<&str>, used: Option<Vec<char>>, unused: char) -> Result<Self, &mut str> {
        match Piece::verify_format(used.clone(), unused) {
            Some(c) => return Err(Box::leak(format!("can't have the same char in used and unused, found : {}", c).into_boxed_str())),
            None => {},
        }
        let mut slots: Vec<Slot> = Vec::new();
        for (y,s) in map.iter().enumerate() {
            for (x,c) in s.chars().enumerate() {
                if c == unused {
                    slots.push(Slot::Unused(Point::new(x as u32, y as u32)));
                    continue;
                }
                match used {
                    Some(ref chars) => {
                        if chars.contains(&c) {
                            slots.push(Slot::Used(Point::new( x as u32, y as u32)));
                        } else {
                            slots.push(Slot::Unknown(Point::new( x as u32, y as u32)));
                        }
                    }
                    None => slots.push(Slot::Used(Point::new(x as u32, y as u32))),
                }
            }
        }
        return Ok(Self{slots});

    }

    fn verify_format(used: Option<Vec<char>>,unused: char) -> Option<char> {
        match used {
            Some(used) => {
                for c_char in used {
                    if unused == c_char {
                        return Some(c_char);
                    }
                }
            }
            None => {},
        }
        None
    }
}