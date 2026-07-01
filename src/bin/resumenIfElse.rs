use std::io;

fn main() {
    let mut n1_input = String::new();
    let mut n2_input = String::new();
    let mut op_input = String::new();
    
    io::stdin().read_line(&mut n1_input).unwrap();
    io::stdin().read_line(&mut n2_input).unwrap();
    io::stdin().read_line(&mut op_input).unwrap();
    
    let n1: f64 = n1_input.trim().parse().unwrap();
    let n2: f64 = n2_input.trim().parse().unwrap();
    let op = op_input.trim();
    
    // Write your code below, use n1, n2 and op
    if op== "+" {
       let suma: f64 = n1+n2;
       println!("{}", suma);
    } else if op == "-"{
        let resta: f64 = n1-n2;
        println!("{}", resta);
    } else if op == "*" {
        let multiply: f64 = n1*n2;
        println!("{}", multiply);
    } else {
        let divide: f64 = n1/n2;
        println!("{}", divide);
    };

}
