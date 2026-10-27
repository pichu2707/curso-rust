// Recibirás tres entradas que representan un perfil de usuario. La primera entrada es un nombre de usuario, la segunda es una edad y la tercera es una dirección de correo electrónico. Valida el perfil según las siguientes reglas e imprime el mensaje correspondiente:
//
// Reglas de validación:
//
// El nombre de usuario debe tener al menos 3 caracteres
// La edad debe estar entre 13 y 120 (inclusive)
// El correo electrónico debe contener el símbolo @
// Requisitos:
//
// Lee la entrada del nombre de usuario y elimina los espacios en blanco
// Lee la entrada de la edad, elimina los espacios en blanco y conviértela a i32
// Lee la entrada del correo electrónico y elimina los espacios en blanco
// Comprueba si la longitud del nombre de usuario es de al menos 3 caracteres
// Comprueba si la edad está en el rango de 13 a 120 (usa match con rangos)
// Comprueba si el correo electrónico contiene el carácter @ (usa el método .contains())
// Si todas las validaciones son correctas, imprime: Valid profile
// Si el nombre de usuario es demasiado corto, imprime: Invalid username
// Si la edad está fuera del rango, imprime: Invalid age
// Si el correo electrónico no contiene @, imprime: Invalid email
// Comprueba las validaciones en orden: primero el nombre de usuario, después la edad y después el correo electrónico
// Entrada:
//
// Primera línea: Un nombre de usuario (por ejemplo, alice123)
// Segunda línea: Una edad (por ejemplo, 25)
// Tercera línea: Una dirección de correo electrónico (por ejemplo, alice@example.com)
// Salida:
//
// Uno de los siguientes valores: Valid profile, Invalid username, Invalid age o Invalid email
//

use std::io;

fn main() {
    // Leer la entrada del nombre de usuario
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");
    let username = username.trim();
    
    // Leer la entrada de la edad
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let age: i32 = age_input.trim().parse().expect("Failed to parse age");
    
    // Leer la entrada del correo electrónico
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Failed to read line");
    let email = email.trim();
    
    // TODO: Escribe tu código a continuación para validar el perfil
    let valid_username = username.len() >= 3;

    let valid_age = match age {
        13..=120 => true,
        _ => false,
    };

    let valid_email = email.contains('@');

    if !valid_username {
        println!("Invalid username");
    } else if !valid_age {
        println!("Invalid age");
    } else if !valid_email {
        println!("Invalid email");
    } else {
        println!("Valid profile");
    }
    
}
