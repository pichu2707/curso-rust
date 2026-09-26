// Recibirás dos entradas. La primera entrada es el nombre de un producto, y la segunda entrada indica si el producto está en stock (yes) o agotado (no). Crea un Option<String> que contenga Some(product_name) si el producto está en stock, o None si está agotado. Usa .expect() con un mensaje descriptivo para extraer el nombre del producto e imprimirlo.
//
// Requisitos:
//
// Lee la primera entrada (nombre del producto) y elimina los espacios en blanco (trim)
// Lee la segunda entrada (estado del stock) y elimina los espacios en blanco (trim)
// Crea una variable Option<String>:
// Si el estado del stock es yes, asigna Some(product_name.to_string())
// Si el estado del stock es no, asigna None
// Usa .expect() con el mensaje "Product should be in stock" para extraer el nombre del producto
// Imprime el nombre del producto extraído en el formato: Product available: [product_name]
// Entrada:
//
// Primera línea: El nombre de un producto (por ejemplo, Laptop)
// Segunda línea: Ya sea yes o no
// Salida:
//
// Si el estado del stock es yes: Product available: [product_name]
// Si el estado del stock es no: El programa entrará en pánico (panic) con el mensaje personalizado "Product should be in stock" (este es el comportamiento esperado para este desafío)
// Nota: Cuando el estado del stock es no, llamar a .expect() sobre None causará un pánico con tu mensaje personalizado. Esto demuestra cómo .expect() proporciona información de error más útil en comparación con .unwrap().
//
use std::io;

fn main() {
    // Leer el nombre del producto
    let mut product_name = String::new();
    io::stdin()
        .read_line(&mut product_name)
        .expect("Failed to read line");
    let product_name = product_name.trim();

    // Leer el estado del stock
    let mut stock_status = String::new();
    io::stdin()
        .read_line(&mut stock_status)
        .expect("Failed to read line");
    let stock_status = stock_status.trim();

    // TODO: Escribe tu código abajo
    // Crear un Option<String> basado en stock_status
    let product_stock: Option<String> = if stock_status == "yes" {
        Some(product_name.to_string())
    } else {
        None
    };
    // Usar .expect() para extraer el nombre del producto
    let excetracted_product = product_stock.expect("Product should be in stock");
    // Imprimir el resultado en el formato: Product available: [product_name]
    println!("Product available: {}", excetracted_product);
}
