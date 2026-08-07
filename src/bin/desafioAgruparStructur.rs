// Recibirás tres entradas: un nombre de producto, un precio (como un número decimal) y una cantidad de stock (como un número entero). Define una estructura Product con tres campos: name de tipo String, price de tipo f64 y stock de tipo i32. Luego, crea una instancia de esta estructura utilizando los valores de entrada e imprime la información del producto.
//
// Requisitos:
//
// Define una estructura Product con los campos: name: String, price: f64 y stock: i32
// Lee la primera entrada como el nombre del producto
// Lee la segunda entrada y conviértela a f64 para el precio
// Lee la tercera entrada y conviértela a i32 para la cantidad de stock
// Crea una instancia de la estructura Product con estos valores
// Imprime la información del producto en el formato exacto que se muestra a continuación
// Entrada:
//
// Primera línea: Nombre del producto (por ejemplo, Laptop)
// Segunda línea: Precio como un número decimal (por ejemplo, 999.99)
// Tercera línea: Cantidad de stock como un número entero (por ejemplo, 15)
// Salida:
//
// Primera línea: Product: [name]
// Segunda línea: Price: $[price]
// Tercera línea: Stock: [quantity]

use std::io;

// TODO: Define tu struct Product aquí

fn main() {
    // Leer el nombre del producto
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim().to_string();

    // Leer el precio
    let mut price_input = String::new();
    io::stdin()
        .read_line(&mut price_input)
        .expect("Failed to read line");
    let price: f64 = price_input.trim().parse().expect("Failed to parse price");

    // Leer la cantidad en stock
    let mut stock_input = String::new();
    io::stdin()
        .read_line(&mut stock_input)
        .expect("Failed to read line");
    let stock: i32 = stock_input.trim().parse().expect("Failed to parse stock");

    // TODO: Crear una instancia de la struct Product usando los valores de entrada
    struct Product {
        name: String,
        price: f64,
        stock: i32,
    }

    // TODO: Imprimir la información del producto en el formato requerido

    println!("Product: {}", name);
    println!("Price: ${}", price);
    println!("Stock: {}", stock);
}
