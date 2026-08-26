// Recibirás un entero n que indica el número de pares estudiante-calificación a procesar. Luego recibirás n pares de entradas: el nombre de un estudiante seguido de su calificación en el examen (como un entero). Crea un mapa hash para almacenar los nombres de los estudiantes como claves y sus calificaciones como valores. Después de insertar todos los pares, itera sobre el mapa hash e imprime el nombre y la calificación de cada estudiante.
//
// Requisitos:
//
// Importa HashMap de std::collections
// Crea un mapa hash mutable con los tipos HashMap<String, i32>
// Lee la primera entrada y conviértela a i32 para obtener el número de pares
// Usa un bucle para leer n pares de entradas (nombre del estudiante, luego calificación)
// Inserta cada nombre de estudiante y calificación en el mapa hash
// Usa un bucle for para iterar sobre el mapa hash con &map
// Imprime la información de cada estudiante en el formato: [name]: [score]
// Imprime los pares en cualquier orden (los mapas hash no garantizan el orden)
// Entrada:
//
// Primera línea: Un entero n (por ejemplo, 3)
// Siguientes n pares de líneas:
// Nombre del estudiante (por ejemplo, Alice)
// Calificación del examen como un entero (por ejemplo, 95)
// Salida:
//
// Una línea por cada estudiante en el formato: [name]: [score]
// El orden de las líneas de salida puede variar entre ejecuciones de prueba
//

use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Leer el número de pares estudiante-calificación
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Crear el HashMap
    let mut students: HashMap<String, i32> = HashMap::new();

    // Leer los n pares
    for _ in 0..n {
        let name: String = lines.next().unwrap().unwrap().trim().to_string();

        let score: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

        students.insert(name, score);
    }

    // Recorrer el HashMap
    for (name, score) in &students {
        println!("{}: {}", name, score);
    }
}
