#![allow(dead_code)]

const DEFAULT_PLAYER1: &str = "@a";
const DEFAULT_PLAYER2: &str = "$s";

#[derive(Debug,PartialEq,Eq,Clone, Copy)]
pub struct Player<'a> {
	id: usize,
	chars: &'a str,
}

impl<'a> Player<'a> {
    pub fn new(id: usize, chars: Option<&'a str>) -> Self {
        let chars = match chars {
            Some(chars) => chars,
            None => match id {
                1 => &DEFAULT_PLAYER1,
                2 => &DEFAULT_PLAYER2,
                _ => panic!("no default chars set for this player"),  
            },
        };
        Self {
            id,
            chars,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn chars(&self) -> &str {
        &self.chars
    }
}