// Recibirás tres entradas: un nombre de producto, su precio (como un número decimal) y otro nombre de producto. Crea un mapa hash mutable para almacenar nombres de productos como claves (tipo String) y sus precios como valores (tipo f64). Inserta el primer producto con su precio en el mapa hash. Luego, inserta el segundo producto con un precio de 0.0. Finalmente, actualiza el precio del segundo producto a 15.99 insertándolo de nuevo con el nuevo precio. Imprime toda la información en el formato exacto que se muestra a continuación.
//
// Nota sobre .clone(): Al insertar una variable String como clave en un HashMap, las reglas de propiedad (ownership) de Rust significan que la variable será movida al mapa y ya no podrá usarse después. Para seguir usando la variable (por ejemplo, para imprimirla o insertarla de nuevo), llama a .clone() sobre ella al pasarla a insert(). Por ejemplo: map.insert(name.clone(), price).
//
// Requisitos:
//
// Importa HashMap de std::collections
// Crea un mapa hash mutable con los tipos HashMap<String, f64>
// Lee la primera entrada como el nombre del primer producto
// Lee la segunda entrada y conviértela a f64 para el precio del primer producto
// Inserta el primer producto y su precio en el mapa hash usando .clone() en el nombre del producto
// Lee la tercera entrada como el nombre del segundo producto
// Inserta el segundo producto con un precio de 0.0 usando .clone() en el nombre del producto
// Inserta el segundo producto nuevamente con un precio de 15.99 usando .clone() (esto sobrescribirá el valor anterior)
// Imprime la información de ambos productos en el formato exacto que se muestra a continuación
// Entrada:
//
// Primera línea: Nombre del primer producto (por ejemplo, Laptop)
// Segunda línea: Precio del primer producto como un número decimal (por ejemplo, 999.99)
// Tercera línea: Nombre del segundo producto (por ejemplo, Mouse)
// Salida:
//
// Primera línea: Inserted [first_product] at $[price]
// Segunda línea: Inserted [second_product] at $0.00
// Tercera línea: Updated [second_product] to $15.99

use std::collections::HashMap;
use std::io;

fn main() {
    // Leer entradas
    let mut product1 = String::new();
    io::stdin()
        .read_line(&mut product1)
        .expect("Failed to read line");
    let product1 = product1.trim().to_string();

    let mut price1_input = String::new();
    io::stdin()
        .read_line(&mut price1_input)
        .expect("Failed to read line");
    let price1: f64 = price1_input.trim().parse().expect("Failed to parse price");

    let mut product2 = String::new();
    io::stdin()
        .read_line(&mut product2)
        .expect("Failed to read line");
    let product2 = product2.trim().to_string();

    // TODO: Escribe tu código abajo
    // Crear un HashMap mutable, insertar productos y mostrar la salida requerida
    let mut products: HashMap<String, f64> = HashMap::new();
    // Insertar primer producto
    products.insert(product1.clone(), price1);
    println!("Inserted {} at ${:.2}", product1, price1);
    // Insertar segundo producto
    products.insert(product2.clone(), 0.0);
    println!("Inserted {} at ${:.2}", product2, 0.0);
    // Actualizar el precio del segundo producto
    products.insert(product2.clone(), 15.99);
    print!("Updated {} to ${:.2}", product2, 15.99);
}
