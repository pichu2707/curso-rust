use std::io;

fn main() {
    let mut input_name = String::new();
    let mut input_year = String::new();

    io::stdin().read_line(&mut input_name).unwrap();
    io::stdin().read_line(&mut input_year).unwrap();

    let name = input_name.trim();
    let year: i32 = input_year.trim().parse().unwrap();

    // Primera letra en mayúsculas
    let first_letter = name.chars().next().unwrap().to_uppercase().to_string();

    // Invertir el año
    let reverse_year = year.to_string().chars().rev().collect::<String>();

    // Código secreto
    let secret_code = format!("⭐{}⭐-{}", first_letter, reverse_year);

    println!("{}", secret_code);
}
