// Crea una estructura Circle con un solo campo: radius (f64).
//
// Añade un bloque de implementación para Circle con dos métodos que utilicen &self:
//
// circumference: devuelve la circunferencia como f64 (fórmula: 2.0 * 3.14159 * radius)
// diameter: devuelve el diámetro como f64 (fórmula: 2.0 * radius)
// Recibirás una entrada:
//
// El radio del círculo (f64)
// Crea una instancia de Circle con el radio proporcionado y, después, imprime la circunferencia y el diámetro en líneas separadas.
//
// Formato de salida esperado:
//
// {circumference}
// {diameter}

use std::io;

// TODO: Define la estructura Circle aquí
struct Circle {
    radius: f64,
}

// TODO: Añade el bloque de implementación para Circle con los métodos circumference y diameter
impl Circle {
    fn circumference(&self) -> f64 {
        self.radius * 2.0 * 3.14159        
    }

    fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
}
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let radius: f64 = input.trim().parse().expect("Invalid number");
    
    // TODO: Crea una instancia de Circle e imprime la circumference y el diameter
    let calculate = Circle { radius};
    println!("{}", calculate.circumference());
    println!("{}", calculate.diameter());
}
