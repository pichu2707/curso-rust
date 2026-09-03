// Recibirás un número como entrada. Crea un Option<i32> que contenga Some(number) si el número es positivo, o None si el número es cero o negativo. Usa .is_some() y .is_none() para verificar el Option e imprimir el mensaje apropiado.
//
// Requisitos:
//
// Lee la entrada y conviértela a i32
// Crea una variable Option<i32>:
// Si el número es positivo (mayor que 0), asigna Some(number)
// Si el número es cero o negativo, asigna None
// Usa .is_some() para verificar si el Option contiene un valor
// Si .is_some() devuelve true, imprime: Positive number detected
// Usa .is_none() para verificar si el Option está vacío
// Si .is_none() devuelve true, imprime: No positive number
// Entrada:
//
// Un solo número entero (por ejemplo, 42, 0, o -5)
// Salida:
//
// Si el número es positivo: Positive number detected
// Si el número es cero o negativo: No positive number
//
use std::io;

fn main() {
    // Leer entrada
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number: i32 = input.trim().parse().expect("Invalid input");

    // TODO: Escribe tu código debajo
    // Crear un Option<i32> basado en si el número es positivo

    let maybe_number: Option<i32> = if number > 0 { Some(number) } else { None };

    if maybe_number.is_some() {
        println!("Positive number detected");
    };
    // Comprobar si el Option es Some e imprimir el mensaje apropiado
    if maybe_number.is_none() {
        println!("No positive number");
    }
}
