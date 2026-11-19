// TODO: Define un enum público TrafficLight con tres variantes:
// - Red(u32)
// - Yellow(u32)
// - Green(u32)

pub enum TrafficLight {
    Red(u32),
    Yellow(u32),
    Green(u32),
}

// TODO: Implementa el enum TrafficLight con un método status
// El método status debe devolver un String con el mensaje apropiado:
// - Red: "Stop for {seconds} seconds"
// - Yellow: "Caution for {seconds} seconds"
// - Green: "Go for {seconds} seconds"
// Usa match para manejar cada variante
impl TrafficLight {
    pub fn status(&self) -> String {
        match self {
            TrafficLight::Red(seconds) => {
                format!("Stop for {} seconds", seconds)
            },
            TrafficLight::Yellow(seconds) => {
                format!("Caution for {} seconds", seconds)
                    },
            TrafficLight::Green(seconds) => {
                format!("Go for {} seconds", seconds)
        },
    }
}
}
