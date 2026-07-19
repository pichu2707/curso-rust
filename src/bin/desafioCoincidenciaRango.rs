// Recibirás una edad como entrada. Lee la entrada, conviértela en un entero y usa una expresión match con patrones de rango para imprimir la categoría de edad adecuada.
//
// Requisitos:
//
// Si la edad es 0..=12, imprime "Child"
// Si la edad es 13..=19, imprime "Teenager"
// Si la edad es 20..=64, imprime "Adult"
// Si la edad es 65..=120, imprime "Senior"
// Para cualquier otra edad, imprime "Invalid age"
// Entrada: Un único número entero que representa la edad
//
// Salida: Imprime la categoría de edad: "Child", "Teenager", "Adult", "Senior", o "Invalid age"

use std::io;

fn main() {
    // Leer la entrada
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let age: i32 = input.trim().parse().expect("Please enter a valid number");

    // TODO: Escribe tu código a continuación usando una expresión match con patrones de rango

    match age {
        0..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        20..=64 => println!("Adult"),
        65..=120 => println!("Senior"),
        _ => println!("Invalid age"),
    }
    // Imprimir el resultado
}
