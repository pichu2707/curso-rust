// Recibirás cuatro entradas: un nombre de jugador, una puntuación inicial (como un entero), una puntuación de bonificación para añadir (como un entero) y un ajuste de puntuación final (como un entero). Define una estructura Player con dos campos: name de tipo String y score de tipo i32. Crea una instancia mutable de esta estructura, luego modifica el campo de puntuación varias veces utilizando los valores de entrada.
//
// Requisitos:
//
// Define una estructura Player con los campos: name: String y score: i32
// Lee la primera entrada como el nombre del jugador
// Lee la segunda entrada y conviértela a i32 para la puntuación inicial
// Crea una instancia mutable de la estructura Player con estos valores
// Lee la tercera entrada y conviértela a i32 para la puntuación de bonificación
// Añade la puntuación de bonificación a la puntuación actual del jugador utilizando la notación de punto
// Lee la cuarta entrada y conviértela a i32 para el ajuste final
// Añade el ajuste final a la puntuación actual del jugador
// Imprime la información del jugador en el formato exacto que se muestra a continuación
// Entrada:
//
// Primera línea: Nombre del jugador (por ejemplo, Alice)
// Segunda línea: Puntuación inicial como un entero (por ejemplo, 50)
// Tercera línea: Puntuación de bonificación como un entero (por ejemplo, 30)
// Cuarta línea: Ajuste final como un entero (por ejemplo, 20)
// Salida:
//
// Primera línea: Player: [name]
// Segunda línea: Final Score: [score]

use std::io;

struct Player {
    name: String,
    score: i32,
}

// TODO: Define la estructura Player aquí con los campos name y score

fn main() {
    // Leer el nombre del jugador
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim().to_string();

    // Leer la puntuación inicial
    let mut initial_score = String::new();
    io::stdin()
        .read_line(&mut initial_score)
        .expect("Failed to read line");
    let initial_score: i32 = initial_score.trim().parse().expect("Invalid number");

    // Leer la puntuación de bonificación
    let mut bonus = String::new();
    io::stdin()
        .read_line(&mut bonus)
        .expect("Failed to read line");
    let bonus: i32 = bonus.trim().parse().expect("Invalid number");

    // Leer el ajuste final
    let mut adjustment = String::new();
    io::stdin()
        .read_line(&mut adjustment)
        .expect("Failed to read line");
    let adjustment: i32 = adjustment.trim().parse().expect("Invalid number");

    // TODO: Crear una instancia mutable de Player

    let mut player = Player {
        name: String::from(name),
        score: initial_score,
    };

    // TODO: Añadir la puntuación de bonificación a la puntuación del jugador

    player.score += bonus;

    // TODO: Añadir el ajuste final a la puntuación del jugador
    player.score += adjustment;

    // TODO: Imprimir la salida en el formato requerido

    println!("Player: {}", player.name);
    println!("Final Score: {}", player.score);
}
