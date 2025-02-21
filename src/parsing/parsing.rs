use regex::Regex;

use super::{player::Player, points::{self, Point}};

pub struct AntField<'a> {
    // player data
    player: Player<'a>,

    // player owned points
    self_owned: Vec<Point>,

    // Ennemies owned points
    ennemie_owned: Vec<Point>,

    // width and length of the AntField
    length: u32,
    width: u32,
}

impl<'a> AntField<'a> {
    pub fn parse(arg: &str) -> Self {
        let lines = arg.split("\n").collect::<Vec<&str>>();
        let mut player_n = match extract_player(lines[0]) {
            Some(v) => v,
            None => panic!("can't parse player number"),
        };
        let p = Player::new(player_n as usize, None);
        let (w,l) = match extract_lenght_width(lines[1]) {
            Some((w,l)) => (w,l),
            None => panic!("can't parse antfield size"),
        };
        let field = match extract_field(lines[3..(3+l) as usize].to_vec()) {
            Some(f) => f,
            None => panic!("can't parse antfield on lines 3 - {}",3+l),
        };
        let player_owned = get_points(field, p.chars());
        let ennemie_owned = get_points(field, "")
        todo!()
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
    let re = Regex::new("^[$]{3} exec p(\\d+)").unwrap();
    let cptr = match re.captures(line) {
        Some(value) => value,
        None => return None,
    };
    match cptr.get(1).unwrap().as_str().parse::<u32>() {
        Ok(v) => Some(v),
        Err(e) => {
            dbg!(e);
            None
        },
    }
}