use std::io;

fn analyze_string(s: &str) {
    // Escribe tu código aquí
    let length = s.len();
    println!("Length: {}", length);
    let fourd_char = s.chars().nth(4).unwrap();
    println!("Char at 4: {}", fourd_char);
    let contains_word = s.contains("Rust");
    println!("Contains Rust: {}", contains_word);
    let ends_with_dot = s.ends_with(".");
    println!("Ends with dot: {}", ends_with_dot);
    let uppercase = s.to_uppercase();
    println!("Uppercase: {}", uppercase);
}

fn main() {
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();
    let message = message.trim();
    analyze_string(message);
}
