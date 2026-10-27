// Escribe una función get_unique_items que reciba numbers y devuelva un vector que contenga solo los valores únicos de la entrada.
//
// Usa un mapa hash para rastrear qué números ya has visto, luego construye un vector de resultados que contenga solo los números que aparecen exactamente una vez en la entrada.
//
// Lógica:
//
// Crea un mapa hash para contar las ocurrencias de cada número
// Itera a través del vector de entrada y actualiza el conteo para cada número
// Crea un vector de resultados
// Itera a través del vector de entrada nuevamente y añade al resultado los números que tengan un conteo de 1
// Devuelve el vector de resultados
// Parámetros:
//
// numbers (Vec<i32>): Un vector de enteros que puede contener duplicados
// Devuelve: Un vector que contiene solo los números que aparecen exactamente una vez en la entrada, en el orden en que aparecieron por primera vez (Vec<i32>)
//

use std::collections::HashMap;

fn get_unique_items(numbers: Vec<i32>) -> Vec<i32> {
    // Escribe el código aquí
    let mut count = HashMap::new();
    for i in &numbers {
        *count.entry(*i).or_insert(0) += 1;
    }
    let mut result = Vec::new();

    for i in &numbers {
        if count.get(i) == Some(&1) {
            result.push(*i);
        }    
    }
    result
}

fn main() {
    let numbers = vec![2, 3, 2, 5, 7, 3, 0];

    let result = get_unique_items(numbers);

    println!("{:?}", result);
}
