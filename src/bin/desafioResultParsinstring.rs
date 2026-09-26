// Escribe una función parse_to_integer que tome una cadena input y devuelva el valor entero analizado. Si la cadena no se puede analizar como un entero válido, devuelve 0 como valor predeterminado.
// Usa el método .parse() para convertir la cadena en un i32 y gestiona Result adecuadamente para proporcionar un valor de retorno seguro.
//
// Parámetros:
//
// input (String): La cadena que se analizará como un entero
// Devuelve: El entero analizado si la operación tiene éxito, o 0 si el análisis falla (i32)
//

use std::io;

fn parse_to_integer(input: String) -> i32 {
    input.trim().parse::<i32>().unwrap_or(0)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer");

    let number = parse_to_integer(input);
    println!("{}", number);
}
