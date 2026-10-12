// Recibirás dos entradas. La primera entrada es una oración completa y la segunda son dos números separados por un espacio que representan los índices inicial y final para crear un segmento. Crea un String a partir de la primera entrada y, después, crea un segmento de cadena utilizando el rango especificado por los dos índices. Imprime la cadena original en la primera línea y el segmento extraído en la segunda línea.
//
// Requisitos:
//
// Lee la primera entrada (la oración completa) y elimina los espacios en blanco
// Crea un String a partir de esta entrada
// Lee la segunda entrada (dos números separados por un espacio) y elimina los espacios en blanco
// Divide la segunda entrada por el espacio para obtener los índices inicial y final
// Convierte ambos índices a usize
// Crea un segmento de cadena utilizando la sintaxis de rango &string[start..end]
// Imprime la cadena original
// Imprime el segmento extraído
// Entrada:
//
// Primera línea: Una oración (por ejemplo, Hello World)
// Segunda línea: Dos números separados por un espacio (por ejemplo, 0 5)
// Salida:
//
// Primera línea: La cadena original
// Segunda línea: El segmento extraído

use std::io;

fn main() {
    // Lee la primera entrada (la oración completa)
    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");
    let sentence = sentence.trim();

    // Lee la segunda entrada (dos números separados por un espacio)
    let mut indices = String::new();
    io::stdin()
        .read_line(&mut indices)
        .expect("Failed to read line");
    let indices = indices.trim();

    // TODO: Escribe tu código debajo
    // Analiza los índices y crea el slice de la cadena

    let parts: Vec<&str> = indices.split_whitespace().collect();
    let start: usize = parts[0].parse().unwrap();
    let end: usize = parts[1].parse().unwrap();

    let slice = &sentence[start..end];
    // Imprime la cadena original y el slice
    println!("{}", sentence);

    println!("{}", slice);
}
