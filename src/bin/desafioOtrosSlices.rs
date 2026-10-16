// Recibirás tres entradas. La primera entrada es una lista de números separados por comas. La segunda entrada consta de dos números separados por un espacio que representan los índices inicial y final para crear un segmento. La tercera entrada es un solo número que representa un índice para acceder al segmento.
//
// Crea un vector a partir de los números separados por comas y, después, crea un segmento utilizando el rango especificado. Accede al elemento en el índice indicado del segmento e imprímelo.
//
// Requisitos:
//
// Lee la primera entrada (números separados por comas) y elimina los espacios en blanco
// Divide la entrada por comas y analiza cada número como i32
// Crea un vector a partir de estos números
// Lee la segunda entrada (dos números separados por un espacio) y elimina los espacios en blanco
// Divide la segunda entrada por el espacio para obtener los índices inicial y final
// Analiza ambos índices como usize
// Crea un segmento a partir del vector utilizando la sintaxis de rango &vec[start..end]
// Lee la tercera entrada (índice al que acceder) y elimina los espacios en blanco
// Analiza el índice como usize
// Accede al elemento en el índice especificado del segmento
// Imprime el elemento al que se accedió
// Entrada:
//
// Primera línea: Números separados por comas (por ejemplo, 10,20,30,40,50)
// Segunda línea: Dos números separados por un espacio (por ejemplo, 1 4)
// Tercera línea: Un número de índice (por ejemplo, 2)
// Salida:
//
// El elemento en el índice especificado del segmento

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Lee la primera entrada (números separados por comas)
    let first_input = lines.next().unwrap().unwrap();

    // Lee la segunda entrada (índices de inicio y fin)
    let second_input = lines.next().unwrap().unwrap();

    // Lee la tercera entrada (índice a acceder)
    let third_input = lines.next().unwrap().unwrap();

    // TODO: Escribe tu código debajo
    // Analiza los números separados por comas y crea un vector
    let numbers: Vec<i32> = first_input
        .trim()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    // Analiza los índices de inicio y fin
    let indices: Vec<usize> = second_input
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let start = indices[0];
    let end = indices[1];
    // Crea un slice a partir del vector
    let slice = &numbers[start..end];
    // Accede al elemento en el índice especificado del slice
    let index: usize = third_input.trim().parse().unwrap();
    let result = slice[index];
    // Imprime el resultado
    println!("{}", result);
    // println!("{}", result);
}
