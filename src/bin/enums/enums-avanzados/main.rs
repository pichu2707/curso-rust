mod command;

use command::Command;

fn main() {
    // Leer entradas
    let mut say_message = String::new();
    std::io::stdin()
        .read_line(&mut say_message)
        .expect("Failed to read line");
    let say_message = say_message.trim().to_string();

    let mut direction = String::new();
    std::io::stdin()
        .read_line(&mut direction)
        .expect("Failed to read line");
    let direction = direction.trim().to_string();

    let mut steps_input = String::new();
    std::io::stdin()
        .read_line(&mut steps_input)
        .expect("Failed to read line");
    let steps: u32 = steps_input.trim().parse().expect("Failed to parse steps");

    let mut num1_input = String::new();
    std::io::stdin()
        .read_line(&mut num1_input)
        .expect("Failed to read line");
    let num1: i32 = num1_input.trim().parse().expect("Failed to parse num1");

    let mut num2_input = String::new();
    std::io::stdin()
        .read_line(&mut num2_input)
        .expect("Failed to read line");
    let num2: i32 = num2_input.trim().parse().expect("Failed to parse num2");

    let say = Command::Say(say_message);
    let movement = Command::Move { direction, steps };
    let calculation = Command::Calculate(num1, num2);

    println!("{}", say.execute());
    println!("{}", movement.execute());
    println!("{}", calculation.execute());
}
