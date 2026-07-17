use std::io;

fn main() {
    let mut input_n = String::new();
    io::stdin().read_line(&mut input_n).unwrap();
    let n: usize = input_n.trim().parse().unwrap();

    if n % 2 == 1 && n >= 1 {
        for i in (1..=n).step_by(2) {
            let piramide = "*".repeat(i);
            println!("{}", piramide);
        }
    }
}
