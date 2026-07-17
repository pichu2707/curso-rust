use std::convert::TryInto;
use std::io;

fn calculate_average_grade(grades: [i32; 8]) -> String {
    // Escribe tu código debajo
    let mut result = 0;
    for i in grades {
        result += i;
        println!("Average grade: {}", result.to_string());
    }
}
fn main() {
    let mut input_str_arr = String::new();
    io::stdin().read_line(&mut input_str_arr).unwrap();
    let arr: [i32; 8] = input_str_arr
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap();
    let res = calculate_average_grade(arr);
    println!("{}", res);
}
