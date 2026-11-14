// TODO: Define una struct pública TemperatureSensor
// - location: String (privada)
// - reading: f64 (privada)

pub struct TemperatureSensor {
    location: String,
    reading: f64,
}
// TODO: Implementa la struct TemperatureSensor
impl TemperatureSensor {
    pub fn new(location: String, reading: f64) -> TemperatureSensor {
        println!("Sensor created for: {}", location);
        TemperatureSensor { location, reading }
    }
    // TODO: Crea un constructor público 'new' que:
    // - Toma un location (String) y un reading (f64)
    // - Prints: "Sensor created for: {location}"
    // - Devuelve el nuevo TemperatureSensor
}
