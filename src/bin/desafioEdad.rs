use std::io;

fn main(){
    let mut age = String::new();
    io::stdin().read_line(&mut age).unwrap();

    let age: i32 = age.trim().parse().unwrap();
    let resta: i32 = 120 - age;
    print!("{} years till 120", resta);
}
