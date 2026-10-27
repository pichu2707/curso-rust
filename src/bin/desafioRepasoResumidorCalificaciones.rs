// Recibirás múltiples entradas que representan las calificaciones de los estudiantes. La primera entrada es un número que indica cuántas calificaciones seguirán. Luego recibirás esa cantidad de valores de calificación (uno por línea). Procesa estas calificaciones y proporciona un resumen completo.
//
// Requisitos:
//
// Lee la primera entrada (número de calificaciones), elimina los espacios en blanco y conviértela a i32
// Crea un vector vacío para almacenar las calificaciones
// Usa un bucle para leer cada calificación, elimina los espacios en blanco, conviértela a i32 y agrégala al vector
// Calcula el promedio de las calificaciones (suma de todas las calificaciones dividida por el conteo)
// Encuentra la calificación más alta en el vector
// Encuentra la calificación más baja en el vector
// Cuenta cuántas calificaciones son aprobatorias (70 o más)
// Cuenta cuántas calificaciones son reprobatorias (menos de 70)
// Imprime los resultados en el formato exacto que se muestra a continuación
// Formato de Salida:
//
// Línea 1: Average: X (donde X es el promedio como un entero)
// Línea 2: Highest: X (donde X es la calificación más alta)
// Línea 3: Lowest: X (donde X es la calificación más baja)
// Línea 4: Passing: X (donde X es el conteo de calificaciones aprobatorias)
// Línea 5: Failing: X (donde X es el conteo de calificaciones reprobatorias)
// Entrada:
//
// Primera línea: Número de calificaciones (por ejemplo, 5)
// Líneas siguientes: Valores de calificación individuales (por ejemplo, 85, 92, 67, 78, 95)
// Salida:
//
// Cinco líneas que muestran el promedio, la más alta, la más baja, el conteo de aprobados y el conteo de reprobados
//
use std::io;

fn main() {
    let mut count_input = String::new();
    io::stdin().read_line(&mut count_input).expect("Failed to read line");
    let count: i32 = count_input.trim().parse().expect("Invalid number");

    let mut grades: Vec<i32> = Vec::new();

    for _ in 0..count {
        let mut grade_input = String::new();
        io::stdin().read_line(&mut grade_input).expect("Failed to read line");
        let grade: i32 = grade_input.trim().parse().expect("Invalid number");
        grades.push(grade);
    }

    // Aquí empieza tu código
    let total: i32 = grades.iter().sum();
    let average = total / count;

    let highest = *grades.iter().max().unwrap();
    let lowest = *grades.iter().min().unwrap();

    let passing_count = grades.iter()
        .filter(|&&grade| grade >= 70)
        .count();

    let failing_count = grades.iter()
        .filter(|&&grade| grade < 70)
        .count();

    println!("Average: {}", average);
    println!("Highest: {}", highest);
    println!("Lowest: {}", lowest);
    println!("Passing: {}", passing_count);
    println!("Failing: {}", failing_count);
}

