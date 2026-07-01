use std::io;

fn main(){
let mut chart = String::new();
    io::stdin().read_line(&mut chart).unwrap();

if chart.trim() == "1" {
    print!("T");
} else {
    println!("F");
};
}

