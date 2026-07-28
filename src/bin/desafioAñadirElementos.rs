// Recibirás tres números como entrada, cada uno en una línea separada. Lee estos números, conviértelos a enteros y añádelos a un vector usando el método .push(). Después de añadir los tres números, imprime cada elemento del vector en una línea separada.
//
// Requisitos:
//
// Crea un vector mutable vacío
// Lee tres números de la entrada y convierte cada uno a un entero
// Usa .push() para añadir cada número al vector
// Imprime cada elemento del vector en una línea separada
// Entrada: Tres números enteros, cada uno en una línea separada
//
// Salida: Imprime cada elemento del vector en una línea separada

use std::io;

fn main() {
    // Leer tres números de la entrada
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");
    let num1: i32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");
    let num2: i32 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    io::stdin()
        .read_line(&mut input3)
        .expect("Failed to read line");
    let num3: i32 = input3.trim().parse().expect("Invalid input");

    // TODO: Escribe tu código a continuación
    // Crear un vector mutable y añadirle los números
    let mut vector_number = Vec::new();
    vector_number.push(num1);
    vector_number.push(num2);
    vector_number.push(num3);

    // Imprimir cada elemento del vector en una línea separada
    for vector in vector_number {
        println!("{}", vector);
    }
}
