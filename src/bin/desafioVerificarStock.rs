//
// Recibirás cuatro entradas. La primera entrada es el nombre de un artículo que se debe comprobar. La segunda entrada es una lista separada por comas de los nombres de los artículos del inventario. La tercera entrada es una lista separada por comas de los precios correspondientes a cada artículo. La cuarta entrada es una lista separada por comas de las cantidades correspondientes a cada artículo. Crea un HashMap para almacenar los datos del inventario, donde la clave sea el nombre del artículo y el valor sea una tupla que contenga el precio y la cantidad (f64, i32). Después, comprueba si el artículo solicitado existe en el inventario usando .get() e imprime la información de existencias correspondiente.
//
// Requisitos:
//
// Importa HashMap desde std::collections
// Lee la primera entrada (el nombre del artículo que se debe comprobar) y elimina los espacios en blanco
// Lee la segunda entrada (los nombres de los artículos separados por comas) y elimina los espacios en blanco
// Lee la tercera entrada (los precios separados por comas) y elimina los espacios en blanco
// Lee la cuarta entrada (las cantidades separadas por comas) y elimina los espacios en blanco
// Crea un HashMap<String, (f64, i32)> mutable para el inventario
// Divide los nombres de los artículos usando comas
// Divide los precios usando comas y convierte cada uno a f64
// Divide las cantidades usando comas y convierte cada una a i32
// Inserta cada artículo en el inventario con su precio y cantidad correspondientes como una tupla
// Usa .get() para comprobar si el artículo solicitado existe en el inventario
// Usa una expresión match para gestionar el Option devuelto por .get()
// En la rama Some, extrae el precio y la cantidad de la tupla e imprime:
// [item_name] is in stock
// Price: $[price]
// Quantity: [quantity]
// En la rama None, imprime: [item_name] is not in stock
// Entrada:
//
// Primera línea: nombre del artículo que se debe comprobar (por ejemplo, Laptop)
// Segunda línea: nombres de los artículos separados por comas (por ejemplo, Laptop,Mouse,Keyboard)
// Tercera línea: precios separados por comas (por ejemplo, 999.99,25.50,75.00)
// Cuarta línea: cantidades separadas por comas (por ejemplo, 15,50,30)
// Salida:
//
// Si el artículo existe en el inventario:
// Primera línea: [item_name] is in stock
// Segunda línea: Price: $[price]
// Tercera línea: Quantity: [quantity]
// Si el artículo no existe: [item_name] is not in stock
//

use std::collections::HashMap;
use std::io;

fn main() {
    // Leer el nombre del artículo a verificar
    let mut item_to_check = String::new();
    io::stdin()
        .read_line(&mut item_to_check)
        .expect("Failed to read line");
    let item_to_check = item_to_check.trim();

    // Leer los nombres de los artículos separados por comas
    let mut items_input = String::new();
    io::stdin()
        .read_line(&mut items_input)
        .expect("Failed to read line");
    let items_input = items_input.trim();

    // Leer los precios separados por comas
    let mut prices_input = String::new();
    io::stdin()
        .read_line(&mut prices_input)
        .expect("Failed to read line");
    let prices_input = prices_input.trim();

    // Leer las cantidades separadas por comas
    let mut quantities_input = String::new();
    io::stdin()
        .read_line(&mut quantities_input)
        .expect("Failed to read line");
    let quantities_input = quantities_input.trim();

    // TODO: Escribe tu código a continuación
    // Crear un HashMap para almacenar los datos del inventario
    let mut inventory: HashMap<String, (f64, i32)> = HashMap::new();
    // Dividir las entradas y poblar el HashMap
    let items: Vec<&str> = items_input.split(',').collect();
    let prices: Vec<f64> = prices_input
        .split(',')
        .map(|price| price.trim().parse::<f64>().unwrap())
        .collect();
    let quantities: Vec<i32> = quantities_input
        .split(',')
        .map(|quantity| quantity.trim().parse::<i32>().unwrap())
        .collect();
    for i in 0..items.len() {
        inventory.insert(items[i].trim().to_string(), (prices[i], quantities[i]));
    }
    // Usar .get() para verificar si el artículo existe
    // Usar match para manejar el Option e imprimir la salida apropiada
    match inventory.get(item_to_check) {
        Some((price, quantity)) => {
            println!("{} is in stock", item_to_check);
            println!("Price: ${}", price);
            println!("Quantity: {}", quantity);
        }
        None => {
            println!("{} is not in stock", item_to_check);
        }
    }
}
