// Recibirás una única entrada que contiene un mensaje. Crea un cierre que no acepte parámetros e imprima este mensaje. Almacena el cierre en una variable y, después, llámalo para mostrar el mensaje.
//
// Requisitos:
//
// Lee el mensaje de entrada y elimina los espacios en blanco
// Crea un cierre usando la sintaxis || que imprima el mensaje
// Almacena el cierre en una variable
// Llama al cierre para ejecutarlo
// Entrada:
//
// Una única línea que contiene un mensaje (por ejemplo, Welcome to Rust!)
// Salida:
//
// El mensaje impreso por el cierre

use std::io;

fn main() {
    // Leer la entrada
    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");
    let message = message.trim();

    // TODO: Crear un cierre que imprima el mensaje y guardarlo en una variable
    let message_clousure = || {
        println!("{}", message);
    };

    // TODO: Llamar al cierre para imprimir el mensaje
    message_clousure();
}
