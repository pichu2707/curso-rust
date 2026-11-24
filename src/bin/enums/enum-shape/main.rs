mod shape;

use shape::Shape;

fn main() {
    // Leer entradas
    let mut radius_input = String::new();
    std::io::stdin().read_line(&mut radius_input).expect("Failed to read line");
    let radius: f64 = radius_input.trim().parse().expect("Invalid number");

    let mut width_input = String::new();
    std::io::stdin().read_line(&mut width_input).expect("Failed to read line");
    let width: f64 = width_input.trim().parse().expect("Invalid number");

    let mut height_input = String::new();
    std::io::stdin().read_line(&mut height_input).expect("Failed to read line");
    let height: f64 = height_input.trim().parse().expect("Invalid number");

    // TODO: Crea una forma Circle usando el radius
    let circle = Shape::Circle(radius);    
    // TODO: Crea una forma Rectangle usando width y height
    let rectangle = Shape::Rectangle { width, height };
    // TODO: Imprime el área de cada forma con un decimal
    // Format: "Circle area: {area}" and "Rectangle area: {area}"
    println!("Circle area: {:.1}", circle.area());
    println!("Rectangle area: {:.1}", rectangle.area());
}

