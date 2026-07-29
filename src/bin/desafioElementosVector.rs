// Escribe una función get_element_at que reciba un vector numbers y un índice idx, y devuelva el elemento en ese índice utilizando un acceso seguro.
//
// Usa el método .get() para acceder de forma segura al elemento. Si el índice existe, devuelve el valor. Si el índice no existe, devuelve -1 como valor predeterminado.
//
// Parámetros:
//
// numbers (Vec<i32>): El vector al que se va a acceder
// idx (i32): El índice a recuperar
// Devuelve: El elemento en el índice dado, o -1 si el índice no existe (i32)

fn get_element_at(numbers: Vec<i32>, idx: i32) -> i32 {
    // Escribe el código aquí
    numbers.get(idx as usize).copied().unwrap_or(-1)
}
