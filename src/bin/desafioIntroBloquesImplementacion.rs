// Crea una estructura Cat con dos campos: name (String) y lives (u8).
//
// Añade un bloque de implementación para Cat con un método llamado meow que reciba &self e imprima el nombre del gato seguido de " says meow!".
//
// Recibirás dos entradas:
//
// Primera línea: el nombre del gato (String)
// Segunda línea: el número de vidas (u8)
// Crea una instancia de Cat con los valores proporcionados y llama al método meow en ella.
//
// Formato de salida esperado:
//
// {name} says meow!


use std::io;

// TODO: Definir la estructura Cat con los campos 'name' (String) y 'lives' (u8)
struct Cat {
    name: String,
    lives: u8,
}

// TODO: Añadir un bloque de implementación para Cat con un método 'meow'
impl Cat {
    fn meow(&self) {
        println!("{} says meow!", self.name);
    }
}

fn main() {
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();
    
    let mut lives_input = String::new();
    io::stdin().read_line(&mut lives_input).expect("Failed to read line");
    let lives: u8 = lives_input.trim().parse().expect("Failed to parse lives");
    
    // TODO: Crear una instancia de Cat y llamar al método meow
    let my_cat = Cat {
        name,
        lives,
        };

my_cat.meow();
        
}
