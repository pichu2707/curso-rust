// Recibirás una única entrada: una lista de descripciones de tareas separadas por comas. Crea un vector mutable vacío para almacenar las tareas, divide la entrada por comas, añade cada tarea al vector e imprime el número total de tareas seguido de cada tarea en una línea separada.
//
// Requisitos:
//
// Lee la entrada que contiene descripciones de tareas separadas por comas (por ejemplo, Buy groceries,Call dentist,Finish homework)
// Crea un vector mutable vacío de tipo Vec<String>
// Divide la cadena de entrada por comas para obtener las tareas individuales
// Usa .push() para añadir cada tarea al vector
// Imprime el número total de tareas en el formato: Total tasks: X
// Imprime cada tarea en una línea separada en el formato: Task: [task description]
// Entrada:
//
// Una sola línea que contiene descripciones de tareas separadas por comas (por ejemplo, Buy groceries,Call dentist,Finish homework)
// Salida:
//
// Primera línea: Total tasks: X donde X es el número de tareas
// Líneas siguientes: Cada tarea impresa como Task: [task description]

use std::io;

fn main() {
    // Leer la entrada
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    // TODO: Escribe tu código a continuación
    // Crear un vector mutable vacío
    let mut actions: Vec<String> = Vec::new();
    // Dividir la entrada por comas
    for action in input.split(',') {
        // Añadimos la tarea al vector
        actions.push(action.trim().to_string());
    }
    // Imprimir el número total de tareas

    println!("Total tasks: {}", actions.len());
    // Imprimir cada tarea
    for action in actions {
        println!("Task: {}", action);
    }
}
