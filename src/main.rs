use std::io::Read;

mod parsing;
mod logic;

pub fn get_stdin() -> String {
    let mut complete_input = String::new();
    let mut end_loop = (false, 0);

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        // if input == "" {
        //     break;
        // }
        
        complete_input += &input;

        if input.starts_with("Piece") {
            let tmp: Vec<&str> = input.split_whitespace().collect();
            let mut t = tmp[tmp.len() -1];
            t = t.trim_matches(':');

            end_loop.0 = true;
            end_loop.1 = t.parse().unwrap();
            
            continue;
        }

        if end_loop.0 {
            end_loop.1 -= 1;

            if end_loop.1 == 0 {
                break;
            }
        }
    }

    return complete_input
}

fn main() {
    println!("Hello, world!");
}
