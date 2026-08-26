// Recibirás dos entradas: el nombre de un país y el nombre de una ciudad capital. Crea un hash map que almacene nombres de países como claves (tipo String) y sus ciudades capitales como valores (tipo String). Inserta el país y su capital en el hash map. Luego, recibirás una tercera entrada con otro nombre de país para buscar. Usa el método .get() para recuperar la capital de este país y maneja ambos casos: cuando el país existe en el mapa y cuando no.
//
// Nota: .get() devuelve un Option — ya sea Some(&value) (una referencia al valor) cuando la clave existe, o None cuando no existe. En un brazo de match, puedes escribir Some(capital) y Rust manejará automáticamente la referencia por ti, de modo que puedes usar capital directamente en tu sentencia de impresión.
//
// Requisitos:
//
// Importa HashMap de std::collections
// Crea un hash map mutable con los tipos HashMap<String, String>
// Lee la primera entrada como el nombre del país
// Lee la segunda entrada como el nombre de la ciudad capital
// Inserta el país y la capital en el hash map
// Lee la tercera entrada como el país a buscar
// Usa .get() para recuperar la capital del país buscado
// Usa match para manejar el Option devuelto por .get()
// Si se encuentra el país, imprime: The capital of [country] is [capital]
// Si no se encuentra el país, imprime: [country] not found in the map
// Entrada:
//
// Primera línea: Nombre del país a insertar (por ejemplo, France)
// Segunda línea: Nombre de la ciudad capital (por ejemplo, Paris)
// Tercera línea: Nombre del país a buscar (por ejemplo, France o Germany)
// Salida:
//
// Si el país buscado existe: The capital of [country] is [capital]
// Si el país buscado no existe: [country] not found in the map
//

use std::collections::HashMap;
use std::io;

fn main() {
    let mut country = String::new();
    io::stdin()
        .read_line(&mut country)
        .expect("Failed to read line");
    let country = country.trim().to_string();

    let mut capital = String::new();
    io::stdin()
        .read_line(&mut capital)
        .expect("Failed to read line");
    let capital = capital.trim().to_string();

    let mut lookup_country = String::new();
    io::stdin()
        .read_line(&mut lookup_country)
        .expect("Failed to read line");
    let lookup_country = lookup_country.trim().to_string();

    let mut capitales: HashMap<String, String> = HashMap::new();

    capitales.insert(country, capital);

    match capitales.get(&lookup_country) {
        Some(found_capital) => {
            println!("The capital of {} is {}", lookup_country, found_capital);
        }
        None => {
            println!("{} not found in the map", lookup_country);
        }
    }
}
