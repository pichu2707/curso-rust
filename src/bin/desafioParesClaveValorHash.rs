// Recibirás un entero n que indica el número de nombres de jugadores a procesar. Luego recibirás n nombres de jugadores como entradas. Crea un mapa hash para rastrear las puntuaciones de los jugadores (tipo HashMap<String, i32>). Para cada nombre de jugador que recibas, usa .entry() con .or_insert() para agregarlos al mapa con una puntuación inicial de 100 solo si aún no existen. Después de procesar todos los nombres, imprime cada jugador y su puntuación.
//
// Requerimientos:
//
// Importa HashMap desde std::collections
// Crea un mapa hash mutable con los tipos HashMap<String, i32>
// Lee la primera entrada y conviértela a i32 para obtener el número de nombres de jugadores
// Usa un bucle para leer n nombres de jugadores
// Para cada nombre de jugador, usa .entry(name).or_insert(100) para agregarlos con una puntuación de 100 si aún no están en el mapa
// Después de procesar todos los nombres, itera sobre el mapa hash e imprime la información de cada jugador en el formato: [name]: [score]
// Entrada:
//
// Primera línea: Un entero n (por ejemplo, 5)
// Siguientes n líneas: Nombres de jugadores (por ejemplo, Alice, Bob, Alice, Charlie, Bob)
// Salida:
//
// Una línea por cada jugador único en el formato: [name]: [score]
// El orden de las líneas de salida puede variar entre las ejecuciones de prueba

use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Leer el número de nombres de jugadores
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Crear un HashMap mutable para almacenar las puntuaciones de los jugadores
    let mut player_scores: HashMap<String, i32> = HashMap::new();

    // TODO: Escribe tu código a continuación
    // Leer n nombres de jugadores y usar .entry().or_insert(100) para añadirlos al mapa
    for _ in 0..n {
        let name = lines.next().unwrap().unwrap().trim().to_string();

        player_scores.entry(name).or_insert(100);
    }

    // Imprimir cada jugador y su puntuación en el formato: [nombre]: [puntuación]
    for (name, score) in &player_scores {
        println!("{}: {}", name, score);
    }
}
