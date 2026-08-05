// Recibirás cuatro entradas que representan un flujo de trabajo completo de una lista de tareas: primero, una lista de tareas iniciales separadas por comas; segundo, una nueva tarea para añadir; tercero, un número de tarea para visualizar (indexación basada en 1); y cuarto, un número de tarea para eliminar (indexación basada en 1). Crea un gestor de lista de tareas completo que procese todas estas operaciones y muestre el estado final.
//
// Requisitos:
//
// Lee la primera entrada que contiene descripciones de tareas separadas por comas (por ejemplo, Buy groceries,Call dentist,Finish homework)
// Divide la cadena por comas y crea un vector mutable con estas tareas iniciales
// Lee la segunda entrada y añade esta nueva tarea al vector usando .push()
// Lee la tercera entrada, conviértela en un entero (número de tarea basado en 1 para visualizar)
// Lee la cuarta entrada, conviértela en un entero (número de tarea basado en 1 para eliminar)
// Elimina la tarea especificada convirtiendo el índice basado en 1 a basado en 0 y usando .remove()
// Imprime Total tasks: X donde X es el número final de tareas
// Imprime Viewing task: Y donde Y es el número de tarea que se solicitó visualizar
// Itera a través de la lista de tareas final e imprime cada tarea con su número (basado en 1)
// Para la tarea que coincida con el número de visualización (después de los ajustes por eliminación), imprime: [X] [task description] (selected)
// Para todas las demás tareas, imprime: [X] [task description]
// Entrada:
//
// Primera línea: Descripciones de tareas separadas por comas (por ejemplo, Buy groceries,Call dentist,Finish homework)
// Segunda línea: Una nueva tarea para añadir (por ejemplo, Study for exam)
// Tercera línea: Un entero que representa el número de tarea a visualizar (indexación basada en 1)
// Cuarta línea: Un entero que representa el número de tarea a eliminar (indexación basada en 1)
// Salida:
//
// Primera línea: Total tasks: X donde X es el número final de tareas
// Segunda línea: Viewing task: Y donde Y es el número de visualización solicitado
// Líneas siguientes: Cada tarea con su número en el formato [X] [task description]
// La tarea seleccionada (que coincida con el número de visualización) debe tener (selected) al final

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Leer las tareas iniciales separadas por comas
    let initial_tasks = lines.next().unwrap().unwrap();

    // Leer la nueva tarea para añadir
    let new_task = lines.next().unwrap().unwrap();

    // Leer el número de tarea para ver (basado en 1)
    let view_number: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Leer el número de tarea para eliminar (basado en 1)
    let remove_number: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // TODO: Escribe tu código abajo
    // 1. Dividir initial_tasks por comas y crear un vector mutable
    let mut tasks: Vec<String> = initial_tasks
        .trim()
        .split(',')
        .map(|task| task.trim().parse::<String>().unwrap())
        .collect();
    // 2. Añadir new_task al vector
    tasks.push(new_task.trim().to_string());
    // 3. Eliminar la tarea en remove_number (convertir a índice basado en 0)
    let index_to_remove = remove_number - 1;
    tasks.remove(index_to_remove);
    // 4. Imprimir el total de tareas y la información de la tarea consultada

    println!("Total tasks: {}", tasks.len());
    println!("Viewing task: {}", view_number);
    // 5. Iterar a través de la lista final e imprimir cada tarea
    for (index, task) in tasks.iter().enumerate() {
        let task_number = index + 1;

        if task_number == view_number {
            println!("[{}] {} (selected)", task_number, task);
        } else {
            println!("[{}] {}", task_number, task);
        }
    }
}
