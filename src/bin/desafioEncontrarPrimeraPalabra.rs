// Recibirás una entrada que contiene una oración con varias palabras separadas por espacios. Encuentra e imprime la primera palabra de la oración usando el slicing de cadenas.
//
// Requisitos:
//
// Lee la oración de entrada y elimina los espacios en blanco
// Encuentra la posición del primer espacio en la oración
// Crea un slicing de cadena desde el principio de la oración hasta (sin incluir) el primer espacio
// Imprime la primera palabra
// Pista: Puedes usar el método .find() para localizar el primer carácter de espacio. Llámalo así: sentence.find(' '), pasando un carácter de espacio como argumento. Este método devuelve un Option que contiene el índice si se encuentra un espacio. Usa .unwrap_or(sentence.len()) para gestionar los casos en los que no haya ningún espacio (una sola palabra), estableciendo como valor predeterminado la longitud completa de la oración.
//
// Entrada:
//
// Una oración con una o más palabras separadas por espacios (por ejemplo, Hello World from Rust)
// Salida:
//
// La primera palabra de la oración

use std::io;

fn main() {
    let mut sentence = String::new();

    io::stdin()
        .read_line(&mut sentence)
        .expect("Error al leer la entrada");

    let sentence = sentence.trim();

    let first_space = sentence.find(' ').unwrap_or(sentence.len());

    let first_word = &sentence[..first_space];

    println!("{}", first_word);
}
