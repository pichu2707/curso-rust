// Recibirás cinco entradas. La primera entrada es el nombre del artículo que se venderá. La segunda entrada es la cantidad que se venderá. La tercera entrada es una lista de nombres de artículos del inventario separados por comas. La cuarta entrada es una lista de precios correspondientes a cada artículo, separados por comas. La quinta entrada es una lista de cantidades correspondientes a cada artículo, separadas por comas. Crea un HashMap para almacenar los datos del inventario, donde la clave sea el nombre del artículo y el valor sea una tupla que contenga el precio y la cantidad (f64, i32). Después, intenta vender la cantidad solicitada del artículo.
//
// Requisitos:
//
// Importa HashMap desde std::collections
// Lee la primera entrada (nombre del artículo que se venderá) y elimina los espacios en blanco
// Lee la segunda entrada (cantidad que se venderá) y elimina los espacios en blanco
// Analiza la cantidad que se venderá como i32
// Lee la tercera entrada (nombres de artículos separados por comas) y elimina los espacios en blanco
// Lee la cuarta entrada (precios separados por comas) y elimina los espacios en blanco
// Lee la quinta entrada (cantidades separadas por comas) y elimina los espacios en blanco
// Crea un HashMap<String, (f64, i32)> mutable para el inventario
// Divide los nombres de los artículos por comas
// Divide los precios por comas y analiza cada uno como f64
// Divide las cantidades por comas y analiza cada una como i32
// Inserta cada artículo en el inventario con su precio y cantidad correspondientes como una tupla
// Usa .get() para comprobar si el artículo solicitado existe en el inventario
// Usa una expresión match para manejar el Option devuelto por .get()
// En la rama Some:
// Extrae el precio y la cantidad actual de la tupla
// Comprueba si la cantidad actual es mayor o igual que la cantidad que se venderá
// Si es así:
// Calcula la nueva cantidad restando la cantidad que se venderá de la cantidad actual
// Calcula el precio total multiplicando el precio del artículo por la cantidad que se venderá
// Usa .insert() para actualizar el inventario con la nueva cantidad
// Imprime: Sold [quantity_to_sell] [item_name]
// Imprime: Total: $[total_price]
// Imprime: Remaining stock: [new_quantity]
// Si no (inventario insuficiente):
// Imprime: Insufficient stock for [item_name]
// Imprime: Available: [current_quantity]
// En la rama None, imprime: [item_name] is not in stock
// Entrada:
//
// Primera línea: nombre del artículo que se venderá (por ejemplo, Mouse)
// Segunda línea: cantidad que se venderá (por ejemplo, 10)
// Tercera línea: nombres de artículos separados por comas (por ejemplo, Laptop,Mouse,Keyboard)
// Cuarta línea: precios separados por comas (por ejemplo, 999.99,25.50,75.00)
// Quinta línea: cantidades separadas por comas (por ejemplo, 15,50,30)
// Salida:
//
// Si el artículo existe y hay suficiente inventario:
// Primera línea: Sold [quantity_to_sell] [item_name]
// Segunda línea: Total: $[total_price]
// Tercera línea: Remaining stock: [new_quantity]
// Si el artículo existe, pero no hay suficiente inventario:
// Primera línea: Insufficient stock for [item_name]
// Segunda línea: Available: [current_quantity]
// Si el artículo no existe: [item_name] is not in stock

use std::collections::HashMap;
use std::io;

fn main() {
    // Leer la entrada
    let mut item_to_sell = String::new();
    io::stdin()
        .read_line(&mut item_to_sell)
        .expect("Failed to read line");
    let item_to_sell = item_to_sell.trim();

    let mut quantity_to_sell_input = String::new();
    io::stdin()
        .read_line(&mut quantity_to_sell_input)
        .expect("Failed to read line");
    let quantity_to_sell: i32 = quantity_to_sell_input
        .trim()
        .parse()
        .expect("Invalid number");

    let mut item_names_input = String::new();
    io::stdin()
        .read_line(&mut item_names_input)
        .expect("Failed to read line");
    let item_names_input = item_names_input.trim();

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

    // TODO: Escribe tu código abajo
    // Crear un HashMap para almacenar el inventario
    let mut inventory: HashMap<String, (f64, i32)> = HashMap::new();
    // Procesar las entradas y construir el inventario
    let items: Vec<&str> = item_names_input.split(',').collect();
    let prices: Vec<f64> = prices_input
        .split(',')
        .map(|price| price.trim().parse::<f64>().unwrap())
        .collect();
    let quantities: Vec<i32> = quantities_input
        .split(',')
        .map(|quantity| quantity.trim().parse::<i32>().unwrap())
        .collect();
    // Meter los artículos en el HashMap
    for (index, item) in items.iter().enumerate() {
        inventory.insert(item.trim().to_string(), (prices[index], quantities[index]));
    }
    // Verificar si el artículo existe y gestiona la venta
    match inventory.get(item_to_sell) {
        Some(&(price, current_quantity)) => {
            if current_quantity >= quantity_to_sell {
                let new_quantity = current_quantity - quantity_to_sell;
                let total_price = price * quantity_to_sell as f64;

                inventory.insert(item_to_sell.to_string(), (price, new_quantity));

                println!("Sold {} {}", quantity_to_sell, item_to_sell);
                println!("Total: ${}", total_price);
                println!("Remaining stock: {}", new_quantity);
            } else {
                println!("Insufficient stock for {}", item_to_sell);
                println!("Available: {}", current_quantity);
            }
        }
        None => {
            println!("{} is not in stock", item_to_sell);
        }
    }
}
