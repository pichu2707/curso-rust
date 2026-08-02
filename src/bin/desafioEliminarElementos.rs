// Recibirás dos entradas: primero, una lista de números separados por comas, y segundo, un índice a eliminar. Lee ambas entradas, crea un vector a partir de los números, elimina el elemento en el índice especificado y imprime los elementos restantes en líneas separadas.
//
// Requerimientos:
//
// Lee la primera entrada que contiene números separados por comas (por ejemplo, 10,20,30,40,50)
// Divide la cadena por comas y convierte cada número en un entero
// Crea un vector mutable a partir de estos números
// Lee la segunda entrada y conviértela en un entero que represente el índice
// Usa .remove() para eliminar el elemento en ese índice
// Imprime cada elemento restante en una línea separada
// Entrada:
//
// Primera línea: Enteros separados por comas (por ejemplo, 10,20,30,40,50)
// Segunda línea: Un entero que representa el índice a eliminar
// Salida: Imprime cada elemento restante del vector en una línea separada

// 5. Imprimir cada elemento restante en una línea separada
use std::io;

fn main() {
    // Leer los números separados por comas
    let mut numbers_input = String::new();
    io::stdin()
        .read_line(&mut numbers_input)
        .expect("Failed to read line");

    // Leer el índice a eliminar
    let mut index_input = String::new();
    io::stdin()
        .read_line(&mut index_input)
        .expect("Failed to read line");

    // Convertir los números en un vector mutable de i32
    let mut numbers: Vec<i32> = numbers_input
        .trim()
        .split(',')
        .map(|number| number.trim().parse::<i32>().unwrap())
        .collect();

    // Convertir el índice a usize
    let index: usize = index_input.trim().parse().expect("El índice no es válido");

    // Eliminar el elemento situado en ese índice
    numbers.remove(index);

    // Imprimir los elementos restantes
    for number in numbers {
        println!("{}", number);
    }
}
