use std::io;

fn main() {
    let mut input_str_arr_1 = String::new();
    let mut input_str_arr_2 = String::new();
    io::stdin().read_line(&mut input_str_arr_1).unwrap();
    io::stdin().read_line(&mut input_str_arr_2).unwrap();
    let arr1: Vec<String> = input_str_arr_1
        .trim()
        .split(',')
        .map(String::from)
        .collect();
    let arr2: Vec<String> = input_str_arr_2
        .trim()
        .split(',')
        .map(String::from)
        .collect();

    let mut result = false;
    // Escribe tu código a continuación

    for i in 0..=arr1.len() - arr2.len() {
        let mut found = true;

        for j in 0..arr2.len() {
            if arr1[i + j] != arr2[j] {
                found = false;
                break;
            }
        }
        if found {
            result = true;
            break;
        }
    }

    println!("{}", result);
}
