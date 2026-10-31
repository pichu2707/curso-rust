// Escribe una función create_adder que tome base_value y devuelva el resultado de sumarle 10 usando un closure.
//
// Dentro de la función, crea un closure que capture base_value del entorno y le sume 10. Llama al closure y devuelve su resultado.
//
// Parámetros:
//
// base_value (i32): El número al que se le sumará 10
// Devuelve: El resultado de sumar 10 a base_value (i32)
//

fn create_adder(base_value: i32) -> i32 {
    // Escribe el código aquí
    let result = || base_value + 10;
    result()
}

fn main() {
    let resultado = create_adder();
    println!("{}", resultado);
}
