// Recibirás dos entradas. La primera entrada es una lista de números separados por comas (por ejemplo, 10,20,30,40), y la segunda entrada es un número objetivo a buscar. Analiza los números separados por comas en un vector, luego busca el número objetivo. Usa .iter().position() para encontrar el índice del número objetivo, el cual devuelve un Option<usize>. Maneja ambos casos usando match e imprime el mensaje apropiado.
//
// Requisitos:
//
// Lee la primera entrada (números separados por comas) y elimina los espacios en blanco
// Divide la entrada por comas y analiza cada parte como i32 para crear un vector
// Lee la segunda entrada (número objetivo) y elimina los espacios en blanco
// Analiza el número objetivo como i32
// Usa .iter().position(|&x| x == target) para buscar el objetivo en el vector
// Usa una expresión match para manejar el Option devuelto por .position()
// En el brazo Some(index), imprime: Found at index [index]
// En el brazo None, imprime: Not found
// Entrada:
//
// Primera línea: Números separados por comas (por ejemplo, 10,20,30,40)
// Segunda línea: Un número objetivo a buscar (por ejemplo, 30)
// Salida:
//
// Si se encuentra el objetivo: Found at index [index]
// Si no se encuentra el objetivo: Not found
//
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Leer los números separados por comas
    let numbers_input = lines.next().unwrap().unwrap().trim().to_string();

    // Leer el número objetivo
    let target_input = lines.next().unwrap().unwrap().trim().to_string();

    // Convertir los números separados por comas en un vector
    let numbers: Vec<i32> = numbers_input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    // Convertir el número objetivo
    let target: i32 = target_input.parse().unwrap();

    // TODO: Escribe tu código abajo
    // Usa .iter().position() para encontrar el objetivo y match para manejar el resultado
    let position: Option<usize> = numbers.iter().position(|&x| x == target);

    match position {
        Some(index) => println!("Found at index {}", index),
        None => println!("Not found"),
    }
}
