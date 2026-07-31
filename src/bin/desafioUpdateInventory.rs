// Escribe una función add_ten_to_all que toma un vector mutable numbers y suma 10 a cada elemento in-place, luego devuelve el vector modificado.
//
// Usa la iteración mutable con &mut para modificar cada elemento directamente. Recuerda usar el operador de desreferencia * para acceder y modificar los valores reales.
//
// Parámetros:
//
// numbers (Vec<i32>): Un vector mutable de enteros a modificar
// Devuelve: El vector modificado con 10 sumado a cada elemento (Vec<i32>)

fn add_ten_to_all(numbers: Vec<i32>) -> Vec<i32> {
    let mut numbers = numbers;
    for number in &mut numbers {
        *number = *number + 10;
    }
    numbers
}
