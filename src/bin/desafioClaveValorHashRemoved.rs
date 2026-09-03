// Recibirás un entero n que indica el número de artículos en un inventario. Luego recibirás n pares de entradas: el nombre de un artículo seguido de su cantidad (como un entero). Después de eso, recibirás una entrada más con el nombre de un artículo para eliminar del inventario. Crea un hash map para almacenar el inventario, inserta todos los artículos, elimina el artículo especificado e imprime el resultado de la eliminación junto con el inventario restante.
//
// Requisitos:
//
// Importa HashMap de std::collections
// Crea un hash map mutable con los tipos HashMap<String, i32>
// Lee la primera entrada y conviértela a i32 para obtener el número de artículos
// Usa un bucle para leer n pares de entradas (nombre del artículo, luego la cantidad)
// Inserta cada artículo y su cantidad en el hash map
// Lee una entrada más como el nombre del artículo a eliminar
// Usa .remove() para eliminar el artículo del hash map
// Usa match para manejar el Option devuelto por .remove()
// Si el artículo fue encontrado y eliminado, imprime: Removed [quantity] [item_name]
// Si el artículo no fue encontrado, imprime: [item_name] not found
// Después de manejar la eliminación, itera sobre el hash map restante e imprime cada artículo en el formato: [item_name]: [quantity]
// Entrada:
//
// Primera línea: Un entero n (por ejemplo, 3)
// Siguientes n pares de líneas:
// Nombre del artículo (por ejemplo, apples)
// Cantidad como un entero (por ejemplo, 50)
// Última línea: Nombre del artículo a eliminar (por ejemplo, bananas)
// Salida:
//
// Primera línea: Ya sea Removed [quantity] [item_name] o [item_name] not found
// Líneas siguientes: Una línea por cada artículo restante en el formato: [item_name]: [quantity]
// El orden de los artículos restantes puede variar entre las ejecuciones de prueba

use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Leer el número de artículos
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Crear el inventario
    let mut inventory: HashMap<String, i32> = HashMap::new();

    // Leer e insertar los artículos
    for _ in 0..n {
        let item_name = lines.next().unwrap().unwrap().trim().to_string();
        let quantity: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

        inventory.insert(item_name, quantity);
    }

    // Leer el artículo que se eliminará
    let item_to_remove = lines.next().unwrap().unwrap().trim().to_string();

    // Eliminarlo y comprobar el resultado
    match inventory.remove(&item_to_remove) {
        Some(quantity) => {
            println!("Removed {} {}", quantity, item_to_remove);
        }
        None => {
            println!("{} not found", item_to_remove);
        }
    }

    // Imprimir el inventario restante
    for (item_name, quantity) in &inventory {
        println!("{}: {}", item_name, quantity);
    }
}
