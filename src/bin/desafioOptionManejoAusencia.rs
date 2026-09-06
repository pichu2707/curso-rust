// Recibirás dos entradas. La primera entrada es un número (como una cadena), y la segunda entrada indica si este número debe tratarse como válido o inválido. Si la segunda entrada es valid, crea un Option<i32> que contenga Some con el número analizado. Si la segunda entrada es invalid, crea un Option<i32> que contenga None. Usa .unwrap() para extraer el valor del Option e imprímelo.
//
// Requerimientos:
//
// Lee la primera entrada y elimina los espacios en blanco
// Lee la segunda entrada y elimina los espacios en blanco
// Analiza la primera entrada a i32 usando .parse::<i32>()
// Crea una variable Option<i32>:
// Si la segunda entrada es valid, asigna Some(parsed_number)
// Si la segunda entrada es invalid, asigna None
// Usa .unwrap() para extraer el valor del Option
// Imprime el valor extraído en el formato: Value: [number]
// Entrada:
//
// Primera línea: Un número como una cadena (por ejemplo, 42)
// Segunda línea: Ya sea valid o invalid
// Salida:
//
// Si la segunda entrada es valid: Value: [number]
// Si la segunda entrada es invalid: El programa entrará en pánico (este es el comportamiento esperado para este desafío)
// Nota: Cuando la segunda entrada es invalid, llamar a .unwrap() sobre None causará un pánico. Esto demuestra la naturaleza peligrosa de .unwrap() cuando no estás seguro de que el Option contiene un valor.
//
use std::io;

fn main() {
    // Leer la primera entrada (número como cadena)
    let mut number_input = String::new();
    io::stdin()
        .read_line(&mut number_input)
        .expect("Failed to read line");
    let number_input = number_input.trim();

    // Leer la segunda entrada (válida o inválida)
    let mut validity_input = String::new();
    io::stdin()
        .read_line(&mut validity_input)
        .expect("Failed to read line");
    let validity_input = validity_input.trim();

    // Analizar el número
    let parsed_number: i32 = number_input.parse().expect("Failed to parse number");

    // TODO: Escribe tu código a continuación
    // Crear un Option<i32> basado en validity_input
    let value_number: Option<i32> = if validity_input == "valid" {
        Some(parsed_number)
    } else {
        None
    };
    // Si validity_input es "valid", asignar Some(parsed_number)
    // Si validity_input es "invalid", asignar None
    // Luego usar .unwrap() para extraer el valor e imprimirlo en el formato: Value: [number]
    let value: i32 = value_number.unwrap();
    println!("Value: {}", value);
}
