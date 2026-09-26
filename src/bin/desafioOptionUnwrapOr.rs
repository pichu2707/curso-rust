// Recibirás dos entradas. La primera entrada es una puntuación (como un número), y la segunda entrada indica si la puntuación está disponible (yes) o falta (no). Crea un Option<i32> basado en el estado de disponibilidad. Usa .unwrap_or() para extraer de forma segura la puntuación con un valor predeterminado de 50, luego imprime la puntuación final.
//
// Requisitos:
//
// Lee la primera entrada (puntuación) y elimina los espacios en blanco
// Convierte la primera entrada a i32
// Lee la segunda entrada (estado de disponibilidad) y elimina los espacios en blanco
// Crea una variable Option<i32>:
// Si el estado de disponibilidad es yes, asigna Some(score)
// Si el estado de disponibilidad es no, asigna None
// Usa .unwrap_or(50) para extraer la puntuación con un valor predeterminado de 50
// Imprime la puntuación final en el formato: Final score: [score]
// Entrada:
//
// Primera línea: Un número que representa la puntuación (por ejemplo, 85)
// Segunda línea: Ya sea yes o no
// Salida:
//
// Si el estado de disponibilidad es yes: Final score: [score] (la puntuación real)
// Si el estado de disponibilidad es no: Final score: 50 (el valor predeterminado)
//
use std::io;

fn main() {
    // Leer la puntuación
    let mut score_input = String::new();
    io::stdin()
        .read_line(&mut score_input)
        .expect("Failed to read line");
    let score: i32 = score_input.trim().parse().expect("Invalid number");

    // Leer el estado de disponibilidad
    let mut availability = String::new();
    io::stdin()
        .read_line(&mut availability)
        .expect("Failed to read line");
    let availability = availability.trim();

    // TODO: Escribe tu código a continuación
    // Crear un Option<i32> basado en el estado de disponibilidad
    let value_number: Option<i32> = if availability == "yes" {
        // Imprimir el resultado
        Some(score)
    } else {
        None
    };
    // Usar .unwrap_or(50) para obtener la puntuación final
    let final_score = value_number.unwrap_or(50);
    // Imprimir el resultado
    println!("Final score: {}", final_score);
}
