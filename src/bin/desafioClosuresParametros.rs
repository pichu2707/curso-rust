// Recibirás dos entradas. La primera entrada es un número y la segunda entrada también es un número. Crea un cierre que acepte dos parámetros y calcule la potencia (el primer parámetro elevado al segundo parámetro). Almacena el cierre en una variable, llámalo con los valores de entrada e imprime el resultado.
//
// Requisitos:
//
// Lee la primera entrada (número base) y elimina los espacios en blanco
// Analízala como i32
// Lee la segunda entrada (exponente) y elimina los espacios en blanco
// Analízala como u32
// Crea un cierre con dos parámetros que calcule la potencia usando el método .pow()
// Almacena el cierre en una variable
// Llama al cierre con ambos valores de entrada
// Imprime el resultado
// Entrada:
//
// Primera línea: un número base (por ejemplo, 2)
// Segunda línea: un exponente (por ejemplo, 3)
// Salida:
//
// El resultado del cálculo de la potencia


use std::io;

fn main() {
    // Leer la primera entrada (número base)
    let mut base_input = String::new();
    io::stdin().read_line(&mut base_input).expect("Failed to read line");
    let base: i32 = base_input.trim().parse().expect("Invalid number");
    
    // Leer la segunda entrada (exponente)
    let mut exp_input = String::new();
    io::stdin().read_line(&mut exp_input).expect("Failed to read line");
    let exponent: u32 = exp_input.trim().parse().expect("Invalid number");
    
    // TODO: Crear un cierre (closure) que calcule la potencia y almacenarlo en una variable
    let potencial = |x: i32, y: u32| x.pow(y);
    // TODO: Llamar al cierre con la base y el exponente
    let result = potencial(base, exponent);
    // Imprimir el resultado
    println!("{}", result);
}
