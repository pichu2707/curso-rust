use std::io;

fn main() {
    // Declara tu variable final_password aquí
    let mut final_password;

    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        final_password = input.trim().to_string();
        // Agrega tu código aquí para manejar la primera entrada
    }

    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        final_password = format!("{}{}", final_password, input.trim().to_string());
    }

    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        final_password = format!("{}{}", final_password, input.trim().to_string());
    }

    println!("Generated password: {}", final_password);
}
