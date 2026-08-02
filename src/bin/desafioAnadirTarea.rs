// Recibirás dos entradas: primero, una lista de tareas existentes separadas por comas, y segundo, una nueva tarea para añadir. Lee ambas entradas, crea un vector a partir de las tareas existentes, añade la nueva tarea al vector e imprime la lista de tareas actualizada.
//
// Requisitos:
//
// Lee la primera entrada que contiene descripciones de tareas separadas por comas (por ejemplo, Buy groceries,Call dentist)
// Divide la cadena por comas para obtener las tareas individuales
// Crea un vector mutable y añade cada tarea existente a él
// Lee la segunda entrada que contiene la nueva tarea para añadir
// Usa .push() para añadir la nueva tarea al vector
// Imprime el número total de tareas en el formato: Total tasks: X
// Imprime cada tarea en una línea separada en el formato: Task: [task description]
// Entrada:
//
// Primera línea: Descripciones de tareas separadas por comas (por ejemplo, Buy groceries,Call dentist)
// Segunda línea: Una nueva tarea para añadir (por ejemplo, Finish homework)
// Salida:
//
// Primera línea: Total tasks: X donde X es el número total de tareas después de añadir la nueva
// Líneas siguientes: Cada tarea impresa como Task: [task description]

use std::io;

fn main() {
    // Leer la primera entrada (tareas separadas por comas)
    let mut existing_tasks = String::new();
    io::stdin()
        .read_line(&mut existing_tasks)
        .expect("Failed to read line");
    let existing_tasks = existing_tasks.trim();

    // Leer la segunda entrada (nueva tarea para añadir)
    let mut new_task = String::new();
    io::stdin()
        .read_line(&mut new_task)
        .expect("Failed to read line");
    let new_task = new_task.trim();

    // TODO: Escribe tu código abajo
    // Dividir existing_tasks por comas y crear un vector mutable
    let mut tasks: Vec<String> = Vec::new();
    for task in existing_tasks.split(',') {
        // Añadir la nueva tarea al vector
        tasks.push(task.trim().to_string());
    }
    tasks.push(new_task.trim().to_string());
    // Imprimir el número total de tareas
    println!("Total tasks: {}", tasks.len());
    // Imprimir cada tarea en el formato requerido
    for action in tasks {
        println!("Task: {}", action);
    }
}
