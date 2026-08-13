// Recibirás cuatro entradas: el ancho de un rectángulo (como un entero), la altura de un rectángulo (como un entero), la longitud del lado de un cuadrado (como un entero) y un número de elección (1 o 2). Define una estructura Rectangle con dos campos: width: i32 y height: i32. Crea dos funciones: una que tome un Rectangle y calcule su área, y otra que tome una longitud de lado y devuelva un Rectangle que represente un cuadrado. Basándote en la entrada de elección, calcula e imprime el área correspondiente.
//
// Requisitos:
//
// Define una estructura Rectangle con los campos: width: i32 y height: i32
// Crea una función calculate_area que tome un Rectangle como parámetro y devuelva un i32 (el área)
// Crea una función create_square que tome un i32 (longitud del lado) como parámetro y devuelva un Rectangle con ancho y alto iguales
// Lee la primera entrada y conviértela a i32 para el ancho del rectángulo
// Lee la segunda entrada y conviértela a i32 para la altura del rectángulo
// Crea una instancia de Rectangle con estas dimensiones
// Lee la tercera entrada y conviértela a i32 para la longitud del lado del cuadrado
// Lee la cuarta entrada y conviértela a i32 para la elección (1 o 2)
// Si la elección es 1: calcula e imprime el área del rectángulo
// Si la elección es 2: crea un cuadrado usando create_square, luego calcula e imprime su área
// Entrada:
//
// Primera línea: Ancho del rectángulo como un entero (por ejemplo, 8)
// Segunda línea: Altura del rectángulo como un entero (por ejemplo, 5)
// Tercera línea: Longitud del lado del cuadrado como un entero (por ejemplo, 6)
// Cuarta línea: Elección como un entero, ya sea 1 o 2 (por ejemplo, 1)
// Salida:
//
// Si la elección es 1: Rectangle area: [area]
// Si la elección es 2: Square area: [area]
//
use std::io;

// TODO: Define la estructura Rectangle aquí

struct Rectangle {
    width: i32,
    height: i32,
}

// TODO: Define la función calculate_area aquí

fn calculate_area(reac: Rectangle) -> i32 {
    reac.width * reac.height
}

// TODO: Define la función create_square aquí

fn create_square(size: i32) -> Rectangle {
    Rectangle {
        width: size,
        height: size,
    }
}
fn main() {
    // Leer el ancho del rectángulo
    let mut width_input = String::new();
    io::stdin()
        .read_line(&mut width_input)
        .expect("Failed to read line");
    let width: i32 = width_input.trim().parse().expect("Invalid input");

    // Leer la altura del rectángulo
    let mut height_input = String::new();
    io::stdin()
        .read_line(&mut height_input)
        .expect("Failed to read line");
    let height: i32 = height_input.trim().parse().expect("Invalid input");

    // Leer la longitud del lado del cuadrado
    let mut side_input = String::new();
    io::stdin()
        .read_line(&mut side_input)
        .expect("Failed to read line");
    let side: i32 = side_input.trim().parse().expect("Invalid input");

    // Leer la opción
    let mut choice_input = String::new();
    io::stdin()
        .read_line(&mut choice_input)
        .expect("Failed to read line");
    let choice: i32 = choice_input.trim().parse().expect("Invalid input");

    // TODO: Escribe tu código abajo
    // Crea una instancia de Rectangle, verifica la opción y calcula el área correspondiente
    let rectangle = Rectangle {
        width: width,
        height: height,
    };

    // Mostrar el resultado basado en la opción
    // Usa: println!("Rectangle area: {}", area); o println!("Square area: {}", area);

    if choice == 1 {
        let area = calculate_area(rectangle);
        println!("Rectangle area: {}", area);
    } else if choice == 2 {
        let square = create_square(side);
        let area = calculate_area(square);
        println!("Square area: {}", area);
    }
}
