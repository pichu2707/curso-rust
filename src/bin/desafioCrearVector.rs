// Recibirás un número como entrada que representa cuántos elementos debe contener un vector. Lee la entrada, conviértela a un entero y crea un vector con esa cantidad de elementos, donde cada elemento sea el número 10.
//
// Requisitos:
//
// Lee la entrada y conviértela a un entero
// Crea un vector usando la macro vec![]
// El vector debe contener el número especificado de elementos
// Cada elemento debe tener el valor 10
// Imprime cada elemento en una línea separada
// Entrada: Un solo entero que representa el número de elementos
//
// Salida: Imprime cada elemento del vector en una línea separada

use std::io;

fn main() {
    // Leer la entrada
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    // TODO: Escribe tu código debajo
    // Crea un vector con n elementos, cada uno con el valor 10

    let numbers = vec![10; n];

    // Imprime cada elemento en una línea separada
    for number in numbers {
        println!("{:?}", number);
    }
}
