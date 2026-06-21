use std::io;

fn bigger(num1: f64, num2: f64) -> f64 {
    // Complete the function
    
    if num1 >= num2 {
        num1 
    } else {
        num2 
    } 
}

fn main() {
    let mut input_iter = String::new();
    let mut input_num1 = String::new();
    let mut input_num2 = String::new();

    io::stdin().read_line(&mut input_iter).unwrap();
    io::stdin().read_line(&mut input_num1).unwrap();
    io::stdin().read_line(&mut input_num2).unwrap();

    let iter: i32 = input_iter.trim().parse().unwrap();
    let mut num1: f64 = input_num1.trim().parse().unwrap();
    let mut num2: f64 = input_num2.trim().parse().unwrap();

    for _ in 0..iter {
        let mut mayor: f64 = bigger(num1, num2);


        if num1 >= num2 {
            mayor /= 2.0;
            num1 = mayor;
            println!("{}", num1);
            if num1 < 2.0 {
                break;
            }
            
        } else {
            mayor /= 2.0;
            num2 = mayor;
            println!("{}", num2);
    if num2 < 2.0 {
                break;
    }
          
        }
    }
}
