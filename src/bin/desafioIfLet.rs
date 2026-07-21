// Recibirás un número como entrada. Lee la entrada, conviértela en un entero y usa una expresión if let para verificar si el número es exactamente 42. Si coincide, imprime "The answer!". Si no coincide, no imprimas nada.
//
// Requisitos:
//
// Usa una expresión if let para verificar si el número es igual a 42
// Si el número es 42, imprime "The answer!"
// Si el número es cualquier otra cosa, no imprimas nada
// Entrada: Un solo número entero
//
// Salida: Imprime "The answer!" si el número es 42, de lo contrario no imprimas nada

use std::io;

fn main() {
    // Leer entrada
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number: i32 = input.trim().parse().expect("Please enter a valid number");

    // TODO: Escribe tu código a continuación
    // Usa if let para verificar si el número es igual a 42
    if let 42 = number {
        println!("The answer!");
    }
}
