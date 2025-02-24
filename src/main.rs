mod parsing;

fn main() {
    let mut line = String::new();
    let mut input: String = String::new();
    loop {
        std::io::stdin().read_line(&mut line).expect("Erreur de lecture");
        let trimmed = line.trim();
        if trimmed.is_empty() {
            // add code
        } else {
            input.push_str("\n");
            input.push_str(trimmed);
        }
        input = String::new();
    }
}