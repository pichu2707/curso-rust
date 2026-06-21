// <Right>Un bucle anidado es simplemente un bucle dentro de otro bucle. El bucle interno completará todas sus iteraciones por cada iteración individual del bucle externo. Una buena analogía para esto es un reloj: por cada hora (bucle externo), el minutero (bucle interno) debe completar su ciclo completo de 60 minutos.
//
// Ejemplo de un bucle anidado:
//
// for x in 0..2 {
//     for y in 0..2 {
//         println!("{} {}", x, y);
//     }
// }
// // Esto producirá:
// // 0 0
// // 0 1
// // 1 0
// // 1 1
// El bucle externo (x) se ejecuta dos veces, y por cada una de esas veces, el bucle interno (y) se ejecuta dos veces.
//
// challenge icon
// Desafío
//
// Principiante
// Escribe un programa que encuentre todas las ternas de números que sumen n utilizando números del 1 al n - 1. El programa debe mostrar todas las combinaciones posibles donde el primer número sea menor o igual que el segundo número, y el segundo número sea menor o igual que el tercer número (es decir, en orden no decreciente). Esto asegura que no tengamos ternas duplicadas como "1 2 7" y "2 1 7", que representan el mismo conjunto de números.
//
// Por ejemplo, si n = 10, la salida debería ser:
//
// 1 1 8
// 1 2 7
// 1 3 6
// 1 4 5
// 2 2 6
// 2 3 5
// 2 4 4
// 3 3 4
// Porque:
//
// 1 + 1 + 8 = 10
// 1 + 2 + 7 = 10
// 1 + 3 + 6 = 10
// 1 + 4 + 5 = 10
// 2 + 2 + 6 = 10
// 2 + 3 + 5 = 10
// 2 + 4 + 4 = 10
// 3 + 3 + 4 = 10
// Orden de impresión: Las ternas deben imprimirse en orden ascendente. El primer número debe iterar de menor a mayor, y para cada primer número, el segundo número debe iterar desde el primer número hasta el valor máximo válido, asegurando que a ≤ b ≤ c.

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    // Write your code below
       for i in 1..=n {
           for j in i..=n {
               for k in j..=n {
                   if i+j+k == n{

                       println!("{} {} {}", i, j, k);
                   }
               } 
           }
       }

   } 
