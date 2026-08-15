// Recibirás tres entradas: un valor rojo (como un entero), un valor verde (como un entero) y un valor azul (como un entero). Define una estructura de tupla Color que contenga tres valores i32 que representen los componentes de color RGB. Crea una instancia de esta estructura de tupla utilizando los valores de entrada, luego accede e imprime cada componente de color utilizando la notación de índice.
//
// Requisitos:
//
// Define una estructura de tupla Color con tres campos i32
// Lee la primera entrada y conviértela a i32 para el valor rojo
// Lee la segunda entrada y conviértela a i32 para el valor verde
// Lee la tercera entrada y conviértela a i32 para el valor azul
// Crea una instancia de la estructura de tupla Color con estos valores
// Accede a cada campo utilizando la notación de punto con índices (0, 1, 2) e imprime la información del color en el formato exacto que se muestra a continuación
// Entrada:
//
// Primera línea: Valor rojo como un entero (por ejemplo, 255)
// Segunda línea: Valor verde como un entero (por ejemplo, 128)
// Tercera línea: Valor azul como un entero (por ejemplo, 0)
// Salida:
//
// Primera línea: Red: [red_value]
// Segunda línea: Green: [green_value]
// Tercera línea: Blue: [blue_value]

use std::io;

// TODO: Define su estructura de tupla Color aquí
struct Color(i32, i32, i32);

fn main() {
    // Leer el valor rojo
    let mut red_input = String::new();
    io::stdin()
        .read_line(&mut red_input)
        .expect("Failed to read line");
    let red: i32 = red_input.trim().parse().expect("Invalid input");

    // Leer el valor verde
    let mut green_input = String::new();
    io::stdin()
        .read_line(&mut green_input)
        .expect("Failed to read line");
    let green: i32 = green_input.trim().parse().expect("Invalid input");

    // Leer el valor azul
    let mut blue_input = String::new();
    io::stdin()
        .read_line(&mut blue_input)
        .expect("Failed to read line");
    let blue: i32 = blue_input.trim().parse().expect("Invalid input");

    // TODO: Crear una instancia de la estructura de tupla Color usando los valores de entrada
    let color = Color(red, green, blue);

    // TODO: Acceder e imprimir cada componente de color usando la notación de índice
    // Formato: Red: [value], Green: [value], Blue: [value]
    println!("Red: {}", color.0);
    println!("Green: {}", color.1);
    println!("Blue: {}", color.2);
}
