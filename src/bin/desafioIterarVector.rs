// Escribe una función sum_all_elements que reciba un vector numbers y devuelva la suma de todos sus elementos.
//
// Usa un bucle for para iterar sobre el vector y acumular la suma de todos los elementos.
//
// Parámetros:
//
// numbers (Vec<i32>): El vector de enteros a sumar
// Devuelve: La suma de todos los elementos en el vector (i32)

fn sum_all_elements(numbers: Vec<i32>) -> i32 {
    // Escribe el código aquí
    let mut result: i32 = 0;
    for number in &numbers {
        result += number;
    }
    result
}
