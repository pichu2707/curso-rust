fn main(){
    let x = 5;
    println!("x is: {}",x);
    {
        let x = 3+x;
        println!("x is: {}", x);
    }
    println!("x is: {}", x);
}
