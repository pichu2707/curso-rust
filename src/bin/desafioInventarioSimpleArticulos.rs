// Recibirás tres entradas. La primera entrada es el nombre de un artículo, la segunda entrada es el precio del artículo y la tercera entrada es la cantidad inicial en stock. Crea un HashMap para almacenar los datos del inventario donde la clave sea el nombre del artículo y el valor sea una tupla que contenga el precio y la cantidad (f64, i32). Agrega el artículo al inventario e imprime un mensaje de confirmación seguido de los detalles actuales del inventario.
//
// Requisitos:
//
// Importa HashMap de std::collections
// Lee la primera entrada (nombre del artículo) y elimina los espacios en blanco
// Lee la segunda entrada (precio) y elimina los espacios en blanco
// Convierte el precio a f64
// Lee la tercera entrada (cantidad) y elimina los espacios en blanco
// Convierte la cantidad a i32
// Crea un HashMap<String, (f64, i32)> mutable para el inventario
// Inserta el artículo en el inventario con el precio y la cantidad como una tupla
// Imprime: Added [item_name] to inventory
// Imprime: Price: $[price]
// Imprime: Quantity: [quantity]
// Entrada:
//
// Primera línea: Nombre del artículo (por ejemplo, Laptop)
// Segunda línea: Precio como un número (por ejemplo, 999.99)
// Tercera línea: Cantidad como un número (por ejemplo, 15)
// Salida:
//
// Primera línea: Added [item_name] to inventory
// Segunda línea: Price: $[price]
// Tercera línea: Quantity: [quantity]

use std::collections::HashMap;
use std::io;

fn main() {
    // Leer el nombre del artículo
    let mut item_name = String::new();
    io::stdin()
        .read_line(&mut item_name)
        .expect("Failed to read line");
    let item_name = item_name.trim().to_string();

    // Leer el precio
    let mut price_input = String::new();
    io::stdin()
        .read_line(&mut price_input)
        .expect("Failed to read line");
    let price: f64 = price_input.trim().parse().expect("Failed to parse price");

    // Leer la cantidad
    let mut quantity_input = String::new();
    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read line");
    let quantity: i32 = quantity_input
        .trim()
        .parse()
        .expect("Failed to parse quantity");

    // TODO: Escribe tu código a continuación
    // Crear un HashMap para almacenar el inventario
    let mut inventory: HashMap<String, (f64, i32)> = HashMap::new();
    // Añadir el artículo al inventario
    inventory.insert(item_name, (price, quantity));
    // Imprimir los mensajes de confirmación
    for (item_name, (price, quantity)) in &inventory {
        println!("Added {} to inventory", item_name);
        println!("Price: ${}", price);
        println!("Quantity: {}", quantity);
    }
}
