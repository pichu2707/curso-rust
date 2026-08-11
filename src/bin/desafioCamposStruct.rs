// Recibirás tres entradas: el nombre del modelo de un coche, el año del coche (como un entero) y su kilometraje (como un entero). Define una estructura Car con tres campos: model de tipo String, year de tipo i32 y mileage de tipo i32. Crea una instancia de esta estructura utilizando los valores de entrada, luego accede e imprime cada campo.
//
// Requisitos:
//
// Define una estructura Car con los campos: model: String, year: i32 y mileage: i32
// Lee la primera entrada como el modelo del coche
// Lee la segunda entrada y conviértela a i32 para el año
// Lee la tercera entrada y conviértela a i32 para el kilometraje
// Crea una instancia de la estructura Car con estos valores
// Usa la notación de punto para acceder a cada campo e imprimir la información del coche en el formato exacto que se muestra a continuación
// Entrada:
//
// Primera línea: Modelo del coche (por ejemplo, Tesla Model 3)
// Segunda línea: Año como un entero (por ejemplo, 2022)
// Tercera línea: Kilometraje como un entero (por ejemplo, 15000)
// Salida:
//
// Primera línea: Car: [model]
// Segunda línea: Year: [year]
// Tercera línea: Mileage: [mileage] km

use std::io;

// TODO: Define la estructura Car aquí con los campos: model, year y mileage

struct Car {
    model: String,
    year: i32,
    mileage: i32,
}

fn main() {
    // Leer el modelo del coche
    let mut model = String::new();
    io::stdin()
        .read_line(&mut model)
        .expect("Failed to read line");
    let model = model.trim().to_string();

    // Leer el año
    let mut year_input = String::new();
    io::stdin()
        .read_line(&mut year_input)
        .expect("Failed to read line");
    let year: i32 = year_input.trim().parse().expect("Invalid year");

    // Leer el kilometraje
    let mut mileage_input = String::new();
    io::stdin()
        .read_line(&mut mileage_input)
        .expect("Failed to read line");
    let mileage: i32 = mileage_input.trim().parse().expect("Invalid mileage");

    // TODO: Crear una instancia de la estructura Car usando los valores de entrada
    let car = Car {
        model: String::from(model),
        year: year,
        mileage: mileage,
    };
    // TODO: Acceder e imprimir cada campo en el formato requerido
    println!("Car: {}", car.model);
    println!("Year: {}", car.year);
    println!("Mileage: {} km", car.mileage);
}
