// Crea una estructura TextStyle con tres campos: bold (bool), italic (bool) y size (u32).
//
// Añade un bloque de implementación para TextStyle con lo siguiente:
//
// new: una función asociada que devuelve un TextStyle con bold establecido en false, italic establecido en false y size establecido en 12
// set_bold: toma &mut self y un valor bool, establece el campo bold y devuelve &mut Self para permitir el encadenamiento
// set_italic: toma &mut self y un valor bool, establece el campo italic y devuelve &mut Self para permitir el encadenamiento
// set_size: toma &mut self y un valor u32, establece el campo size y devuelve &mut Self para permitir el encadenamiento
// Recibirás tres entradas:
//
// Primera línea: configuración de negrita (true o false)
// Segunda línea: configuración de cursiva (true o false)
// Tercera línea: tamaño de fuente (u32)
// Crea un TextStyle mutable usando new y, después, usa el encadenamiento de métodos para aplicar las tres configuraciones en una sola expresión. Imprime el estado final en el formato que se muestra a continuación.
//
// Formato de salida esperado:
//
// Bold: {bold}, Italic: {italic}, Size: {size}

use std::io;

// TODO: Define la struct TextStyle aquí

struct TextStyle {
    bold: bool,
    italic: bool,
    size: u32,
}

// TODO: Añade el bloque de implementación para TextStyle con:
// - función asociada new()
// - método set_bold()
// - método set_italic()
// - método set_size()

impl TextStyle {
    fn new() -> TextStyle {
        TextStyle {
            bold: false,
            italic: false,
            size: 12,
        }
    }

    fn set_bold(&mut self, value: bool) -> &mut Self {
        self.bold = value;
        self
    }

    fn set_italic(&mut self, value: bool) -> &mut Self {
        self.italic = value;
        self
    }

    fn set_size(&mut self, value: u32) -> &mut Self {
        self.size = value;
        self
    }
}

fn main() {
    let mut input = String::new();

    // Leer configuración bold
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let bold: bool = input.trim().parse().expect("Invalid bool");
    input.clear();

    // Leer configuración italic
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let italic: bool = input.trim().parse().expect("Invalid bool");
    input.clear();

    // Leer configuración size
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let size: u32 = input.trim().parse().expect("Invalid u32");

    // TODO: Crea un TextStyle mutable usando new(), luego usa el encadenamiento de métodos
    // para aplicar las tres configuraciones en una sola expresión
    let mut style = TextStyle::new();

    style.set_bold(bold).set_italic(italic).set_size(size);

    // TODO: Imprime el resultado en el formato: Bold: {bold}, Italic: {italic}, Size: {size}
    println!(
        "Bold: {}, Italic: {}, Size: {}",
        style.bold, style.italic, style.size
    );
}
