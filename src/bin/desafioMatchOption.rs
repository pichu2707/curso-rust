// Recibirás el nombre de un estudiante como entrada. El nombre puede estar presente en el sistema o puede faltar. Crea un Option<String> que contenga Some(name) si el nombre no está vacío, o None si el nombre está vacío. Usa una expresión match para manejar ambos casos e imprimir el mensaje apropiado.
//
// Requisitos:
//
// Lee la entrada como una cadena (nombre del estudiante)
// Elimina cualquier espacio en blanco de la entrada usando .trim()
// Crea una variable Option<String>:
// Si el nombre recortado no está vacío, asigna Some(name.to_string())
// Si el nombre recortado está vacío, asigna None
// Usa una expresión match para manejar ambas variantes del Option
// En el brazo Some(name), imprime: Welcome, [name]!
// En el brazo None, imprime: No name provided
// Entrada:
//
// Una sola línea que contiene el nombre de un estudiante o una línea vacía
// Salida:
//
// Si se proporciona un nombre: Welcome, [name]!
// Si no se proporciona ningún nombre (entrada vacía): No name provided

use std::io;

fn main() {
    // Leer la entrada
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let name = input.trim();

    // TODO: Crear un Option<String> basado en si el nombre está vacío o no
    let name_option = if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    };

    match name_option {
        Some(name) => println!("Welcome, {}!", name),
        None => println!("No name provided"),
    }
    // TODO: Usar una expresión match para manejar los casos Some y None e imprimir el mensaje apropiado
}
