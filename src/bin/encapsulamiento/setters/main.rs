mod thermostat;

use thermostat::Thermostat;

fn main() {
    // Leer entradas
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let initial_temp: i32 = input1.trim().parse().expect("Invalid number");

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let first_attempt: i32 = input2.trim().parse().expect("Invalid number");

    let mut input3 = String::new();
    std::io::stdin().read_line(&mut input3).expect("Failed to read line");
    let second_attempt: i32 = input3.trim().parse().expect("Invalid number");

    // TODO: Crea un termostato con la temperatura inicial
    let mut t = Thermostat::new(initial_temp);

    // TODO: Imprime la temperatura inicial
    // Formato: "Initial: {temperature}"
    println!("Initial: {}", t.temperature());
    // TODO: Intenta establecer la primera temperatura e imprime el resultado
    // Format: "After setting to {attempted_value}: {temperature}"
    t.set_temperature(first_attempt);
    println!("After setting to {}: {}", first_attempt, t.temperature());
    // TODO: Intenta establecer la segunda temperatura e imprime el resultado
    // Format: "After setting to {attempted_value}: {temperature}"
    t.set_temperature(second_attempt);
    println!("After setting to {}: {}", second_attempt, t.temperature());
}

