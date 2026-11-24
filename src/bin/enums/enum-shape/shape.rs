// TODO: Define un enum público Shape con dos variantes:
// - Circle: contiene un solo f64 (radius) usando sintaxis de tupla
// - Rectangle: contiene campos con nombre width (f64) y height (f64)

pub enum Shape {
    // TODO: Define la variante Circle con sintaxis de tupla
    Circle(f64),    
    // TODO: Define la variante Rectangle con campos con nombre
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    // TODO: Implementa el método area que devuelve f64
    // Usa coincidencia de patrones para desestructurar cada variante
    // Circle area: 3.14159 * radius * radius
    // Rectangle area: width * height
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius)=>3.14159 * radius * radius,
            Shape::Rectangle { width, height } => width * height,
        }
        // TODO: Haz match sobre self y calcula el área apropiada
    }
}

