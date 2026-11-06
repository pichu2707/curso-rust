// Crea una estructura Rectangle con dos campos: width (u32) y height (u32).
//
// Añade un bloque de implementación para Rectangle con lo siguiente:
//
// new: una función asociada que recibe width: u32 y height: u32, y devuelve un nuevo Rectangle
// area: recibe &self y devuelve el área como u32
// scale: recibe &mut self y un factor: u32, y multiplica tanto el ancho como la altura por el factor
// Recibirás tres entradas:
//
// Primera línea: el ancho (u32)
// Segunda línea: la altura (u32)
// Tercera línea: el factor de escala (u32)
// Crea un Rectangle mutable usando Rectangle::new con las dimensiones proporcionadas. Imprime el área inicial, después escala el rectángulo según el factor dado e imprime el área nueva.
//
// Formato de salida esperado:
//
// {initial_area}
// {scaled_area}
//
//
use std::io;

// TODO: Define la struct Rectangle aquí

// TODO: Añade el bloque de implementación para Rectangle con:
// - new: función asociada que toma width y height, devuelve Rectangle
// - area: método que toma &self, devuelve u32
// - scale: método que toma &mut self y factor, multiplica width y height

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        let result = self.width * self.height;
        return result;
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    let mut input = String::new();

    // Lee width
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let width: u32 = input.trim().parse().expect("Invalid number");

    input.clear();

    // Lee height
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let height: u32 = input.trim().parse().expect("Invalid number");

    input.clear();

    // Lee el factor de escala
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let factor: u32 = input.trim().parse().expect("Invalid number");

    // TODO: Crea un Rectangle mutable usando Rectangle::new
    let mut rectangle = Rectangle::new(width, height);
    // TODO: Imprime el área inicial
    println!("{}", rectangle.area());

    // TODO: Escala el rectángulo por el factor
    rectangle.scale(factor);
    // TODO: Imprime el área nueva
    println!("{}", rectangle.area());
}
