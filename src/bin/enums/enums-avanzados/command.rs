// TODO: Define un enum público Command con tres variantes:
// - Say: contiene un solo String (sintaxis de tupla)
// - Move: contiene campos con nombre 'direction' (String) y 'steps' (u32)
// - Calculate: contiene dos valores i32 (sintaxis de tupla)

pub enum Command {
    // TODO: Define la variante Say
    Say(String),
    // TODO: Define la variante Move con campos con nombre
    Move { direction: String, steps: u32 },
    // TODO: Define la variante Calculate
    Calculate(i32, i32),
}

impl Command {
    // TODO: Implementa el método execute que devuelve un String
    pub fn execute(&self) -> String {
        match self {
            Command::Say(message) => {
                format!("Saying: {}", message)
            }
            Command::Move { direction, steps } => {
                format!("Moving {} steps {}", steps, direction)
            }
            Command::Calculate(a, b) => {
                let sum = a + b;
                format!("{} + {} = {}", a, b, sum)
            }
        }
    }
}
