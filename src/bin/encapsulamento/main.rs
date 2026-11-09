mod profile;

use profile::UserProfile;

fn main() {
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let username = input1.trim().to_string();

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let age: u32 = input2.trim().parse().expect("Failed to parse age");

    // TODO: Crea un UserProfile usando el constructor
    let p = UserProfile::new(username, age);
    println!("Username: {}", p.username());
    println!("Age: {}", p.age());
    // TODO: Usa los métodos getter para imprimir el username y la age
    // Formato:
    // Username: {username}
    // Age: {age}
}

