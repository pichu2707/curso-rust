// Recibirás un número inicial como entrada. Lee la entrada, conviértela en un entero y utiliza un bucle como expresión para encontrar el primer número divisible por 7 empezando desde ese número.
//
// Requisitos:
//
// Usa una expresión loop que devuelva un valor
// Empieza a comprobar desde el número de entrada
// Encuentra el primer número que sea divisible por 7 (el resto es 0 al dividir por 7)
// Usa break para devolver el número encontrado desde el bucle
// Imprime el resultado
// Entrada: Un único entero que representa el número inicial
//
// Salida: Imprime el primer número divisible por 7 empezando desde el número de entrada
//

use std::io;

fn main() {
    // Leer la entrada
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut start_number: i32 = input.trim().parse().expect("Invalid input");

    // TODO: Escribe tu código a continuación
    // Usa una expresión de bucle para encontrar el primer número divisible por 7
    let result = loop {
        if start_number % 7 == 0 {
            break start_number;
        }
        start_number += 1;
    };

    // Imprimir el resultado
    println!("{}", result);
}
