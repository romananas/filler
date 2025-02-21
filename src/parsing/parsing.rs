use super::{player::Player, points::{self, Point}};

pub struct AntField<'a> {
    // player data
    player: Player<'a>,

    // player owned points
    pub self_owned: Vec<Point>,

    // Ennemies owned points
    pub ennemie_owned: Vec<Point>,

    // width and length of the AntField
    pub length: u32,
    pub width: u32,
}

impl<'a> AntField<'a> {
    pub fn parse(arg: &str) -> Self {
        let lines = arg.split("\n").collect::<Vec<&str>>();
        let player_n = match extract_player(lines[0]) {
            Some(v) => v,
            None => panic!("can't parse player number"),
        };
        let p = Player::new(player_n as usize, None);
        let p2 = match p.id() {
            1 => 2,
            _ => 1,
        };
        let adv = Player::new(p2, None);
        let (w,l) = match extract_lenght_width(lines[1]) {
            Some((w,l)) => (w,l),
            None => panic!("can't parse antfield size"),
        };
        let field = match extract_field(lines[3..(3+l) as usize].to_vec()) {
            Some(f) => f,
            None => panic!("can't parse antfield on lines 3 - {}",3+l),
        };
        let player_owned = get_points(field.clone(), p.chars());
        let ennemie_owned = get_points(field, adv.chars());
        Self {
            player: p,
            self_owned: player_owned,
            ennemie_owned: ennemie_owned,
            length: l,
            width: w,
        }
    }
}

pub fn get_points(field: Vec<&str>,chars: &str) -> Vec<Point>{
    let mut result = Vec::new();
    for (y,l) in field.iter().enumerate() {
        for (x,c) in l.chars().enumerate() {
            for to_cmp in chars.chars() {
                if c == to_cmp {
                    result.push(Point::new(x as u32, y as u32));
                }
            }
        }
    }
    result
}

pub fn extract_field(lines: Vec<&str>) -> Option<Vec<&str>> {
    let splited = lines.iter().map(|s| s.split_whitespace().collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    for values in splited.clone() {
        if values.len() != 2 {
            return None;
        }
        if values[0].contains(|c: char| !c.is_numeric()) {
            return None;
        }
    }
    let r = splited.iter().map(|v| v[1]).collect::<Vec<&str>>();
    return Some(r);
}

pub fn extract_lenght_width(line: &str) -> Option<(u32,u32)> {
    let tmp = line.split_whitespace().collect::<Vec<&str>>();
    if tmp.len() != 3{
        return None;
    }
    if tmp[0] != "Antfield" || !tmp[2].ends_with(":") {
        return None;
    }
    match (tmp[1].parse::<u32>(),tmp[2][0..tmp[2].len() -1].parse::<u32>()) {
        (Ok(w),Ok(l)) => Some((w,l)),
        _ => None,
    }
}

pub fn extract_player(line: &str) -> Option<u32>{
    let tmp = line.split_ascii_whitespace().collect::<Vec<&str>>();
    if tmp.len() != 5 {
        return None;
    }
    if tmp[0] != "$$$" || tmp[1] != "exec" || !tmp[2].starts_with("p") {
        return None;
    }
    match tmp[2][1..].parse::<u32>() {
        Ok(v) => Some(v),
        Err(e) => {
            panic!("{e}");
        }
    }
}