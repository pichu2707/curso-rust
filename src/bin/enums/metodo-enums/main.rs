mod traffic;

use traffic::TrafficLight;

fn main() {
    // Leer la entrada para las duraciones
    let mut red_input = String::new();
    std::io::stdin().read_line(&mut red_input).expect("Failed to read line");
    let red_duration: u32 = red_input.trim().parse().expect("Invalid number");

    let mut yellow_input = String::new();
    std::io::stdin().read_line(&mut yellow_input).expect("Failed to read line");
    let yellow_duration: u32 = yellow_input.trim().parse().expect("Invalid number");

    let mut green_input = String::new();
    std::io::stdin().read_line(&mut green_input).expect("Failed to read line");
    let green_duration: u32 = green_input.trim().parse().expect("Invalid number");

    // TODO: Crea instancias de cada variante de TrafficLight con las duraciones de entrada
    let red = TrafficLight::Red(red_duration);
    let yellow = TrafficLight::Yellow(yellow_duration);
    let green = TrafficLight::Green(green_duration);
    // TODO: Llama al método status en cada luz e imprime el resultado
    println!("{}", red.status());
    println!("{}", yellow.status());
    println!("{}", green.status());
}

