mod pet;

use pet::Pet;

fn main() {
    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim().to_string();

    // TODO: Crea un nuevo Pet con el nombre dado
    let pet = Pet::new(name);

    // TODO: Imprime el mensaje de bienvenida usando el getter del nombre del pet
    // Formato: "Welcome, {name}!"
    println!("Welcome, {}!", pet.name());
}
