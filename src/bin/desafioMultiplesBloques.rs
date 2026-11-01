// Crea una estructura Player con dos campos: name (String) y score (u32).
//
// Organiza los métodos de la estructura en dos bloques de implementación separados:
//
// Primer bloque impl: constructor y consulta:
//
// new: una función asociada que recibe un name: String y devuelve un Player con score establecido en 0
// get_score: recibe &self y devuelve la puntuación actual como u32
// Segundo bloque impl: modificaciones de la puntuación:
//
// add_points: recibe &mut self y points: u32, y suma los puntos a la puntuación
// reset_score: recibe &mut self y vuelve a establecer la puntuación en 0
// Recibirás tres entradas:
//
// Primera línea: el nombre del jugador (String)
// Segunda línea: puntos que se añadirán en la primera ronda (u32)
// Tercera línea: puntos que se añadirán en la segunda ronda (u32)
// Crea un jugador usando Player::new, añade los puntos de la primera ronda, imprime la puntuación, después reinicia la puntuación, añade los puntos de la segunda ronda e imprime la puntuación final.
//
// Formato de salida esperado:
//
// {score_after_first_round}
// {score_after_reset_and_second_round}

use std::io;

// TODO: Define la struct Player con los campos name (String) y score (u32)
struct Player {
    name: String,
    score: u32,
}

// TODO: Primer bloque impl - Métodos de constructor y consulta (new, get_score)
impl Player {
    fn new(name: String) -> Player {
        Player { name, score: 0 }
    }
    fn get_score(&self) -> u32 {
        self.score
    }
}

// TODO: Segundo bloque impl - Métodos de modificación de puntuación (add_points, reset_score)
impl Player {
    fn add_points(&mut self, points: u32) {
        self.score += points;
    }

    fn reset_score(&mut self) {
        self.score = 0;
    }
}

fn main() {
    let mut input = String::new();

    // Lee el nombre del jugador
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let name = input.trim().to_string();

    // Lee los puntos de la primera ronda
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let first_round: u32 = input.trim().parse().expect("Invalid number");

    // Lee los puntos de la segunda ronda
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let second_round: u32 = input.trim().parse().expect("Invalid number");

    // TODO: Crea un jugador usando Player::new
    let mut player = Player::new(name);
    // TODO: Añade los puntos de la primera ronda e imprime la puntuación
    player.add_points(first_round);
    println!("{}", player.get_score());
    // TODO: Reinicia la puntuación
    player.reset_score();
    // TODO: Añade los puntos de la segunda ronda e imprime la puntuación final
    player.add_points(second_round);
    println!("{}", player.get_score());
}
