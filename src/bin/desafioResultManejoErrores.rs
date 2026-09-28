// Recibirás dos entradas. La primera entrada es un número que representa el dividendo y la segunda entrada es un número que representa el divisor. Crea una función safe_divide que acepte dos parámetros f64 y devuelva un Result<f64, &'static str>. La función debe devolver Err("Cannot divide by zero") cuando el divisor sea cero, y Ok con el resultado de la división en caso contrario. Usa una expresión match para manejar el resultado e imprimir la salida correspondiente.
//
// Requisitos:
//
// Lee la primera entrada (dividendo) y elimina los espacios en blanco
// Convierte la primera entrada a f64
// Lee la segunda entrada (divisor) y elimina los espacios en blanco
// Convierte la segunda entrada a f64
// Crea una función safe_divide que acepte dos parámetros f64 y devuelva Result<f64, &'static str>
// Dentro de la función, comprueba si el divisor es 0.0:
// Si es así, devuelve Err("Cannot divide by zero")
// Si no, devuelve Ok(dividend / divisor)
// Llama a la función con los dos números convertidos
// Usa una expresión match para manejar el Result
// En el brazo Ok(value), imprime: Division result: [value]
// En el brazo Err(error), imprime: Error: [error]
// Entrada:
//
// Primera línea: Un número que representa el dividendo (por ejemplo, 20.0)
// Segunda línea: Un número que representa el divisor (por ejemplo, 4.0)
// Salida:
//
// Si la división se realiza correctamente: Division result: [result]
// Si se divide entre cero: Error: Cannot divide by zero

use std::io;

fn safe_divide(dividend: f64, divisor: f64) -> Result<f64, &'static str> {
    if divisor == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    // Leer la primera entrada (dividendo)
    let mut dividend_input = String::new();
    io::stdin()
        .read_line(&mut dividend_input)
        .expect("Failed to read line");
    let dividend: f64 = dividend_input.trim().parse().expect("Invalid number");

    // Leer la segunda entrada (divisor)
    let mut divisor_input = String::new();
    io::stdin()
        .read_line(&mut divisor_input)
        .expect("Failed to read line");
    let divisor: f64 = divisor_input.trim().parse().expect("Invalid number");

    let result = safe_divide(dividend, divisor);
    // Imprimir la salida apropiada basada en el resultado
    match result {
        Ok(value) => println!("Division result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}
