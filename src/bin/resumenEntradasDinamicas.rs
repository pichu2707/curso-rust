use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let count: i32 = input.trim().parse().unwrap();
    // Write your code below
    let mut result: i32 = 0;
    for _i in 0..count {
        let mut sumatorio = String::new();
        io::stdin().read_line(&mut sumatorio).unwrap();
        let sumatorio_number: i32 = sumatorio.trim().parse().unwrap();
        result += sumatorio_number;
    }
    println!("{}", result);
}
