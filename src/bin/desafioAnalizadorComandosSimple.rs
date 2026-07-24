// Recibirás una cadena de comando como entrada. Lee la entrada y utiliza las técnicas de flujo de control que has aprendido para analizar y responder a diferentes comandos.
//
// Requisitos:
//
// Lee una cadena de comando desde la entrada
// Usa una expresión match para manejar los siguientes comandos:
// "start" → imprime "System starting..."
// "stop" → imprime "System stopping..."
// "pause" o "wait" → imprime "System paused"
// "status" → imprime "System running"
// Cualquier otro comando → imprime "Unknown command"
// Entrada: Una única cadena de comando
//
// Salida: Imprime la respuesta adecuada basada en el comando

use std::io;

fn main() {
    // Leer el comando de la entrada
    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");
    let command = command.trim();

    // TODO: Escribe tu código a continuación
    // Usa una expresión match para manejar diferentes comandos
    match command {
        "start" => println!("System starting..."),
        "stop" => println!("System stpping..."),
        "pause" | "wait" => println!("System paused"),
        "status" => println!("System running"),
        _ => println!("Unknown command"),
    }
}
