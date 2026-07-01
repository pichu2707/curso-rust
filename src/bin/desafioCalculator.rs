use std::io;

fn main(){
    println!("Calculator App");
    // num1 f64
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).unwrap();
    let num1: f64 = num1.trim().parse().unwrap(); 
    // num2 f64
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).unwrap();
    let num2: f64 = num2.trim().parse().unwrap(); 
    // suma
    println!("Sum: {:.2}",num1+num2);
    // resta
    println!("Difference: {:.2}", num1-num2);
    // multiplicación
    println!("Product: {:.2}", num1*num2);
    // división
    println!("Quotient: {:.2}", num1/num2);
}
