//Crea una función llamada convert_and_print que reciba tres argumentos:
// Un slice de cadena s: &str — este es un número escrito como texto (por ejemplo, "123.67"). Solo se utiliza cuando to_string es false.
// Un flotante n: f64 — un número de punto flotante. Solo se utiliza cuando to_string es true.
// Un booleano to_string: bool — determina qué operación realizar.
// La función debe realizar las siguientes operaciones:
//
// Si to_string es true: convierte n a una cadena usando .to_string(), luego cuenta cuántos caracteres de dígitos contiene (no cuentes el punto . ni el signo menos -). Imprime en el formato:
// Number: [string], Digits: [num of digits]
//
// Por ejemplo, si n = -321.198623, su forma de cadena es "-321.198623", que tiene 9 caracteres de dígitos (sin contar - y .).
//
// Sugerencia: convierte n a una cadena, luego usa .len() en ella y resta 1 por cada . o - que contenga.
// Si to_string es false: analiza la cadena s como un f64 usando .parse(), luego conviértela a i32 (lo que elimina la parte decimal). Imprime en el formato:
// String as number: [number]
//
// Por ejemplo, si s = "123.67231", la salida es String as number: 123.
// Llama a convert_and_print dos veces:
//
// Primero, con to_string establecido en false — esto utiliza la cadena s e ignora n.
// Segundo, con to_string establecido en true — esto utiliza el número n e ignora s.

use std::io;

fn convert_and_print(s: &str, n: f64, to_string: bool) {
    // Escribe tu código aquí
    if to_string {
        // 1. Convertir n a cadena
        let n_str = n.to_string();

        // 2. Calcular el número de dígitos (restando '.' y '-')
        let mut num_digits = n_str.len();

        if n_str.contains('.') {
            num_digits -= 1;
        }
        if n_str.contains('-') {
            num_digits -= 1;
        }

        // 3. Imprimir el resultado en el formato solicitado
        println!("Number: {}, Digits: {}", n_str, num_digits);
    } else {
        // 1. Analizar la cadena s como f64
        let parsed_f64: f64 = s.parse().unwrap();

        // 2. Convertir a i32 (elimina la parte decimal)
        let as_i32 = parsed_f64 as i32;

        // 3. Imprimir el resultado en el formato soliticado
        println!("String as number: {}", as_i32);
    }
}

fn main() {
    let mut input_number_str = String::new();
    let mut input_n = String::new();

    // Leer el primer input (el string para la conversión a número)
    io::stdin().read_line(&mut input_number_str).unwrap();
    // Leer el segundo input (el número flotante para la conversión a string)
    io::stdin().read_line(&mut input_n).unwrap();

    // Limpia espacions y saltos de línea, y parsear n
    let n: f64 = input_n.trim().parse().unwrap();
    let number_str = input_number_str.trim();

    // Llamar a convert_and_print con to_string = false
    convert_and_print(number_str, n, false);

    // Llamar a convert_and_print con to_string = true
    convert_and_print(number_str, n, true);
}
