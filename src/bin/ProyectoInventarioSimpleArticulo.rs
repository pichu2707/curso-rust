// Recibirás seis entradas. La primera entrada es un comando que puede ser add, check o sell. La segunda entrada es el nombre de un artículo. La tercera entrada es un número que representa la cantidad a vender (para el comando sell) o el precio inicial (para el comando add). La cuarta entrada es una lista de nombres de artículos en el inventario separados por comas. La quinta entrada es una lista de precios correspondientes a cada artículo separados por comas. La sexta entrada es una lista de cantidades correspondientes a cada artículo separados por comas.
//
// Construye un sistema de gestión de inventario completo que maneje las tres operaciones: añadir artículos, verificar existencias y vender artículos. Usa un HashMap para almacenar los datos del inventario donde la clave es el nombre del artículo y el valor es una tupla que contiene el precio y la cantidad (f64, i32).
//
// Requisitos:
//
// Importar HashMap de std::collections
// Leer la primera entrada (comando) y eliminar los espacios en blanco
// Leer la segunda entrada (nombre del artículo) y eliminar los espacios en blanco
// Leer la tercera entrada (valor de cantidad/precio) y eliminar los espacios en blanco
// Convertir la tercera entrada a f64
// Leer la cuarta entrada (nombres de artículos separados por comas) y eliminar los espacios en blanco
// Leer la quinta entrada (precios separados por comas) y eliminar los espacios en blanco
// Leer la sexta entrada (cantidades separadas por comas) y eliminar los espacios en blanco
// Crear un HashMap<String, (f64, i32)> mutable para el inventario
// Dividir los nombres de los artículos por comas
// Dividir los precios por comas y convertir cada uno a f64
// Dividir las cantidades por comas y convertir cada una a i32
// Insertar cada artículo en el inventario con su precio y cantidad correspondientes como una tupla
// Usar una expresión match en el comando para determinar qué operación realizar
// Para el comando add:
// Usar .entry() y .or_insert() para añadir el artículo si no existe, usando la tercera entrada como el precio y 0 como la cantidad inicial
// Imprimir: Added [item_name] to inventory
// Imprimir: Price: $[price]
// Imprimir: Quantity: 0
// Para el comando check:
// Usar .get() para verificar si el artículo existe
// Usar una expresión match para manejar el Option
// Si es Some, imprimir:
// [item_name] is in stock
// Price: $[price]
// Quantity: [quantity]
// Si es None, imprimir: [item_name] is not in stock
// Para el comando sell:
// Convertir la tercera entrada a i32 como la cantidad a vender
// Usar .get() para verificar si el artículo existe
// Usar una expresión match para manejar el Option
// Si es Some:
// Verificar si la cantidad actual es suficiente
// Si es así, actualizar el inventario con la nueva cantidad e imprimir:
// Sold [quantity_to_sell] [item_name]
// Total: $[total_price]
// Remaining stock: [new_quantity]
// Si no, imprimir:
// Insufficient stock for [item_name]
// Available: [current_quantity]
// Si es None, imprimir: [item_name] is not in stock
// Para cualquier otro comando, imprimir: Invalid command
// Entrada:
//
// Primera línea: Comando (add, check o sell)
// Segunda línea: Nombre del artículo (por ejemplo, Tablet)
// Tercera línea: Número (precio para add, cantidad para sell, ignorado para check)
// Cuarta línea: Nombres de artículos separados por comas (por ejemplo, Laptop,Mouse,Keyboard)
// Quinta línea: Precios separados por comas (por ejemplo, 999.99,25.50,75.00)
// Sexta línea: Cantidades separadas por comas (por ejemplo, 15,50,30)
// Salida:
//
// Para el comando add:
// Added [item_name] to inventory
// Price: $[price]
// Quantity: 0
// Para el comando check (el artículo existe):
// [item_name] is in stock
// Price: $[price]
// Quantity: [quantity]
// Para el comando check (el artículo no existe): [item_name] is not in stock
// Para el comando sell (existencias suficientes):
// Sold [quantity_to_sell] [item_name]
// Total: $[total_price]
// Remaining stock: [new_quantity]
// Para el comando sell (existencias insuficientes):
// Insufficient stock for [item_name]
// Available: [current_quantity]
// Para el comando sell (el artículo no existe): [item_name] is not in stock
// Para un comando no válido: Invalid command

use std::collections::HashMap;
use std::io;

fn main() {
    // Leer entradas
    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");
    let command = command.trim();

    let mut item_name = String::new();
    io::stdin()
        .read_line(&mut item_name)
        .expect("Failed to read line");
    let item_name = item_name.trim();

    let mut value_input = String::new();
    io::stdin()
        .read_line(&mut value_input)
        .expect("Failed to read line");
    let value_input = value_input.trim();

    let mut items_input = String::new();
    io::stdin()
        .read_line(&mut items_input)
        .expect("Failed to read line");
    let items_input = items_input.trim();

    let mut prices_input = String::new();
    io::stdin()
        .read_line(&mut prices_input)
        .expect("Failed to read line");
    let prices_input = prices_input.trim();

    let mut quantities_input = String::new();
    io::stdin()
        .read_line(&mut quantities_input)
        .expect("Failed to read line");
    let quantities_input = quantities_input.trim();

    // Crear el HashMap del inventario
    let mut inventory: HashMap<String, (f64, i32)> = HashMap::new();

    // TODO: Escribe tu código a continuación
    // Analizar las entradas separadas por comas y poblar el inventario
    let value: f64 = value_input.parse().expect("Invalid number");
    let item_names: Vec<&str> = items_input.split(',').collect();
    let prices: Vec<f64> = prices_input
        .split(',')
        .map(|price| price.parse::<f64>().expect("Invalid price"))
        .collect();
    let quantities: Vec<i32> = quantities_input
        .split(',')
        .map(|quantity| quantity.parse::<i32>().expect("Invalid quantity"))
        .collect();
    for i in 0..item_names.len() {
        inventory.insert(item_names[i].to_string(), (prices[i], quantities[i]));
    }
    // Usar una expresión match en command para manejar las operaciones add, check y sell
    match command {
        "add" => {
            inventory.entry(item_name.to_string()).or_insert((value, 0));
            println!("Added {} to inventory", item_name);
            println!("Price: ${}", value);
            println!("Quantity: 0");
        }
        "check" => match inventory.get(item_name) {
            Some((price, quantity)) => {
                println!("{} is in stock", item_name);
                println!("Price: ${}", price);
                println!("Quantity: {}", quantity);
            }
            None => {
                println!("{} is not in stock", item_name);
            }
        },
        "sell" => {
            let quantity_to_sell: i32 = value_input.parse().expect("Invalid quantity");

            match inventory.get(item_name) {
                Some((price, current_quantity)) => {
                    let price = *price;
                    let current_quantity = *current_quantity;

                    if current_quantity >= quantity_to_sell {
                        let new_quantity = current_quantity - quantity_to_sell;
                        let total_price = price * quantity_to_sell as f64;

                        inventory.insert(item_name.to_string(), (price, new_quantity));

                        println!("Sold {} {}", quantity_to_sell, item_name);
                        println!("Total: ${}", total_price);
                        println!("Remaining stock: {}", new_quantity);
                    } else {
                        println!("Insufficient stock for {}", item_name);
                        println!("Available: {}", current_quantity);
                    }
                }
                None => {
                    println!("{} is not in stock", item_name);
                }
            }
        }
        _ => {
            println!("Invalid command");
        }
    }
}

