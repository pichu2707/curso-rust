// Declara el módulo product
mod product;

// TODO: Trae Product al ámbito con una sentencia use
use product::Product;

fn main() {
    // Lee la entrada
    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim().to_string();

    let mut price_input = String::new();
    std::io::stdin()
        .read_line(&mut price_input)
        .expect("Failed to read line");
    let price: f64 = price_input.trim().parse().expect("Failed to parse price");

    // TODO: Crea un nuevo Product usando la función Product::new
    let product = Product::new(name, price);
    // TODO: Imprime los detalles del producto en el formato:
    // Product: {name}, Price: ${price}
    println!("Product: {}, Price: {}", product.name, product.price);
}
