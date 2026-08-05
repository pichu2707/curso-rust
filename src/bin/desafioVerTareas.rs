// Recibirás dos entradas: primero, una lista de tareas existentes separadas por comas, y segundo, un número de tarea para visualizar (indexación basada en 1). Lee ambas entradas, crea un vector a partir de las tareas y muestra todas las tareas con sus números, resaltando la tarea solicitada.
//
// Requisitos:
//
// Lee la primera entrada que contiene descripciones de tareas separadas por comas (por ejemplo, Buy groceries,Call dentist,Finish homework)
// Divide la cadena por comas para obtener las tareas individuales
// Crea un vector y añade cada tarea a él
// Lee la segunda entrada y conviértela en un entero (el número de tarea a resaltar, usando indexación basada en 1)
// Imprime el número total de tareas en el formato: Total tasks: X
// Itera a través del vector e imprime cada tarea con su número (basado en 1)
// Para el número de tarea solicitado, imprímelo en el formato: [X] [task description] (selected)
// Para todas las demás tareas, imprímelas en el formato: [X] [task description]
// Entrada:
//
// Primera línea: Descripciones de tareas separadas por comas (por ejemplo, Buy groceries,Call dentist,Finish homework)
// Segunda línea: Un entero que representa el número de tarea a resaltar (indexación basada en 1)
// Salida:
//
// Primera línea: Total tasks: X donde X es el número total de tareas
// Líneas siguientes: Cada tarea con su número en el formato [X] [task description]
// La tarea seleccionada debe tener (selected) añadido al final

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Leer las tareas separadas por comas
    let tasks_input = lines.next().unwrap().unwrap();

    // Leer el número de tarea a resaltar
    let task_number: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Dividir la entrada y crear un vector de textos
    let tasks: Vec<String> = tasks_input
        .trim()
        .split(',')
        .map(|task| task.trim().to_string())
        .collect();

    // Imprimir el número total de tareas
    println!("Total tasks: {}", tasks.len());

    // Recorrer las tareas junto con su posición
    for (index, task) in tasks.iter().enumerate() {
        let displayed_number = index + 1;

        if displayed_number == task_number {
            println!("[{}] {} (selected)", displayed_number, task);
        } else {
            println!("[{}] {}", displayed_number, task);
        }
    }
}
