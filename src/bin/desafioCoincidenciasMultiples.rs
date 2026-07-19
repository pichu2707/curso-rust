// Recibirás un número de día (1-7) como entrada, que representa un día de la semana. Lee la entrada, conviértela a un entero y utiliza una expresión match con el operador | para imprimir si es un día laborable o fin de semana.
//
// Requisitos:
//
// Si el día es 1, 2, 3, 4, o 5, imprime "Weekday"
// Si el día es 6 o 7, imprime "Weekend"
// Para cualquier otro número, imprime "Invalid day"
// Entrada: Un único número entero que representa el número del día
//
// Salida: Imprime ya sea "Weekday", "Weekend", o "Invalid day"

use std::io;

fn main() {
    // Leer entrada
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let day: i32 = input.trim().parse().expect("Please enter a number");

    // TODO: Escribe tu código abajo usando la expresión match con el operador |
    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }
}
