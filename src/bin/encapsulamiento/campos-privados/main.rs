mod sensor;

use sensor::TemperatureSensor;

fn main() {
    // Leer entrada
    let mut location = String::new();
    std::io::stdin()
        .read_line(&mut location)
        .expect("Failed to read line");
    let location = location.trim().to_string();

    let mut reading_str = String::new();
    std::io::stdin()
        .read_line(&mut reading_str)
        .expect("Failed to read line");
    let reading: f64 = reading_str.trim().parse().expect("Failed to parse reading");

    // TODO: Crea un TemperatureSensor usando el constructor
    let _sensor = TemperatureSensor::new(location, reading);
    // La función new se encargará de imprimir el mensaje de confirmación
}
