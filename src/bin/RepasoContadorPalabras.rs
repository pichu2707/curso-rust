// Recibirás una oración como entrada. Cuenta cuántas veces aparece cada palabra en la oración y almacena los conteos en un hash map. Luego, imprime cada palabra única junto con su conteo.
//
// Requisitos:
//
// Importa HashMap de std::collections
// Crea un hash map mutable con los tipos HashMap<String, i32>
// Lee la oración de entrada como una sola línea
// Divide la oración en palabras individuales usando .split_whitespace()
// Para cada palabra, usa .entry() con .or_insert(0) para inicializar el conteo a 0 si la palabra no existe
// Después de usar .or_insert(0), incrementa el conteo en 1 para esa palabra
// Después de procesar todas las palabras, itera sobre el hash map e imprime cada palabra con su conteo en el formato: [word]: [count]
// Entrada:
//
// Una sola línea que contiene una oración con palabras separadas por espacios (por ejemplo, hello world hello rust world hello)
// Salida:
//
// Una línea por cada palabra única en el formato: [word]: [count]
// El orden de las líneas de salida puede variar entre las ejecuciones de prueba
//
//
// Recibirás una oración como entrada. Cuenta cuántas veces aparece cada palabra en la oración y almacena los conteos en un hash map. Luego, imprime cada palabra única junto con su conteo.
//
// Requisitos:
//
// Importa HashMap de std::collections
// Crea un hash map mutable con los tipos HashMap<String, i32>
// Lee la oración de entrada como una sola línea
// Divide la oración en palabras individuales usando .split_whitespace()
// Para cada palabra, usa .entry() con .or_insert(0) para inicializar el conteo a 0 si la palabra no existe
// Después de usar .or_insert(0), incrementa el conteo en 1 para esa palabra
// Después de procesar todas las palabras, itera sobre el hash map e imprime cada palabra con su conteo en el formato: [word]: [count]
// Entrada:
//
// Una sola línea que contiene una oración con palabras separadas por espacios (por ejemplo, hello world hello rust world hello)
// Salida:
//
// Una línea por cada palabra única en el formato: [word]: [count]
// El orden de las líneas de salida puede variar entre las ejecuciones de pruebai

use std::collections::HashMap;
use std::io;

fn main() {
    // Leer la oración de entrada
    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");
    let sentence = sentence.trim();

    // Crear un mapa hash mutable para almacenar el conteo de palabras
    let mut word_count: HashMap<String, i32> = HashMap::new();

    // TODO: Escribe tu código a continuación
    // Dividir la oración en palabras y contar cada palabra
    for word in sentence.split_whitespace() {
        let count = word_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    // Imprimir cada palabra con su conteo
    for (word, count) in &word_count {
        println!("{}: {}", word, count);
    }
}
