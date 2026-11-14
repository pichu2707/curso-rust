mod locker;

use locker::Locker;

fn main() {
    // Leer entradas
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let initial_code: u32 = input.trim().parse().expect("Invalid number");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let contents = input.trim().to_string();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let first_attempt: u32 = input.trim().parse().expect("Invalid number");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let new_code: u32 = input.trim().parse().expect("Invalid number");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let second_attempt: u32 = input.trim().parse().expect("Invalid number");

    // TODO: Crea un nuevo Locker con initial_code y contents

    let mut locker = Locker::new(initial_code, contents);
    // TODO: Intenta obtener contents con el código first_attempt
    // Imprime: "Attempt with {code}: {result}"
    let result = locker.get_contents(first_attempt);
    println!("Attempt with {}: {}", first_attempt, result);
    // TODO: Cambia el código del locker a new_code
    // Print: "Code changed"
    locker.set_code(new_code);
    println!("Code changed");
    // TODO: Intenta obtener contents con el código second_attempt
    // Imprime: "Attempt with {code}: {result}"
    let result = locker.get_contents(second_attempt);
    println!("Attempt with {}: {}", second_attempt, result);
}
