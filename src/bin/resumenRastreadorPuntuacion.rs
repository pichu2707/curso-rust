// Recibirás dos entradas: primero, una lista de puntuaciones de exámenes separadas por comas, y segundo, una puntuación para añadir. Crea un vector a partir de las puntuaciones iniciales, añade la nueva puntuación, calcula el promedio de todas las puntuaciones e imprime los resultados.
//
// Requisitos:
//
// Lee la primera entrada que contiene puntuaciones separadas por comas (por ejemplo, 85,92,78)
// Divide la cadena por comas y elimina los espacios en blanco (trim) de cada token individual antes de convertirlo a un entero (por ejemplo, usa .trim() en cada elemento después de dividir)
// Crea un vector mutable a partir de estas puntuaciones
// Lee la segunda entrada y conviértela en un entero (elimina los espacios en blanco primero)
// Usa .push() para añadir la nueva puntuación al vector
// Calcula el promedio de todas las puntuaciones en el vector
// Imprime el número total de puntuaciones
// Imprime la puntuación promedio (como un entero)
// Entrada:
//
// Primera línea: Enteros separados por comas que representan las puntuaciones iniciales (por ejemplo, 85,92,78)
// Segunda línea: Un entero que representa la nueva puntuación a añadir
// Salida:
//
// Primera línea: El número total de puntuaciones
// Segunda línea: La puntuación promedio (como un entero, truncado)
// Nota: Llama siempre a .trim() en toda la línea de entrada y en cada token de puntuación individual después de dividir por comas. Las cadenas de entrada pueden contener espacios en blanco invisibles o caracteres de nueva línea que causarán un error (panic) al analizar si no se eliminan.

use std::io;

fn main() {
    // Leer las puntuaciones separadas por comas
    let mut scores_input = String::new();
    io::stdin()
        .read_line(&mut scores_input)
        .expect("Failed to read line");

    // Leer la nueva puntuación para añadir
    let mut new_score_input = String::new();
    io::stdin()
        .read_line(&mut new_score_input)
        .expect("Failed to read line");

    // TODO: Escribe tu código abajo
    // 1. Dividir scores_input por comas y convertir a enteros
    // 2. Crear un vector mutable a partir de estas puntuaciones
    let mut scores: Vec<i32> = scores_input
        .trim()
        .split(',')
        .map(|score| score.trim().parse::<i32>().unwrap())
        .collect();

    // 3. Convertir new_score_input a un entero
    let new_scores: i32 = new_score_input
        .trim()
        .parse()
        .expect("La puntuación no es válida");
    // 4. Añadir la nueva puntuación al vector
    scores.push(new_scores);
    // 5. Calcular el promedio de todas las puntuaciones
    let mut total = 0;
    for score in &scores {
        total += score;
    }

    let average = total / scores.len() as i32;
    // Imprimir el número total de puntuaciones
    println!("{}", scores.len());

    // Imprimir la puntuación promedio
    println!("{}", average);
}
