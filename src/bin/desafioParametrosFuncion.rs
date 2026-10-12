// Recibirás dos entradas. La primera entrada es un mensaje de texto y la segunda entrada es la longitud máxima permitida para el mensaje. Crea una función llamada truncate_message que acepte como parámetros un segmento de cadena (&str) y una longitud máxima (usize). Si la longitud del mensaje supera la longitud máxima, la función debe devolver un segmento del mensaje hasta la longitud máxima. De lo contrario, debe devolver el mensaje completo. Imprime el resultado.
//
// Requisitos:
//
// Lee la primera entrada (el mensaje de texto) y elimina los espacios en blanco
// Lee la segunda entrada (la longitud máxima) y elimina los espacios en blanco
// Convierte la longitud máxima a usize
// Define una función truncate_message que acepte dos parámetros:
// Un segmento de cadena &str para el mensaje
// Un usize para la longitud máxima
// Dentro de la función, comprueba si la longitud del mensaje es mayor que la longitud máxima
// Si es así, devuelve un segmento del mensaje desde el índice 0 hasta la longitud máxima usando &message[0..max_len]
// Si no, devuelve el mensaje completo
// Llama a la función con el mensaje de entrada y la longitud máxima
// Imprime el resultado devuelto
// Entrada:
//
// Primera línea: Un mensaje de texto (por ejemplo, Hello World from Rust)
// Segunda línea: Longitud máxima como número (por ejemplo, 11)
// Salida:
//
// El mensaje truncado si supera la longitud máxima, o el mensaje completo en caso contrario

use std::io;

// todo: define la función truncate_message aquí
fn truncate_message(message: &str, max_len: usize) -> &str {
    if message.len() > max_len {
        &message[0..max_len]
    } else {
        message
    }
}

fn main() {
    // leer el mensaje de texto
    let mut message = string::new();
    io::stdin()
        .read_line(&mut message)
        .expect("failed to read line");
    let message = message.trim();

    // leer la longitud máxima
    let mut max_length_input = string::new();
    io::stdin()
        .read_line(&mut max_length_input)
        .expect("failed to read line");
    let max_len: usize = max_length_input.trim().parse().expect("invalid number");

    // todo: llamar a la función truncate_message y almacenar el resultado

    let message = message.trim();

    let max_len: usize = max_length_input.trim().parse().expect("invalid number");
    // todo: imprimir el resultado
    let result = truncate_message(message, max_len);
    println!("{}", result);
}
