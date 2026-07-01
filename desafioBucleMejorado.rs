fn main() {
    let mut prices = [2.75, 1.50, 5.00, 3.5, 4.1, 2.25, 7.9];

    println!("Original Prices:");
    for (index, value) in prices.iter().enumerate() {
        let number = index + 1;
        println!("Item {}: ${:.2}", number, value);
    }

    println!("\nBundle Deals:");
    for (i, chunk) in prices.chunks(2).enumerate() {
        let numero = i + 1;

        // Evaluamos si el bloque tiene 2 elementos o solo 1
        if chunk.len() == 2 {
            println!(
                "Bundle {}: ${:.2} + ${:.2} = ${:.2}",
                numero,
                chunk[0],
                chunk[1],
                chunk[0] + chunk[1]
            );
        } else {
            // Esto se ejecuta para el último elemento suelto (7.9)
            println!(
                "Bundle {}: ${:.2} (Sin pareja, precio individual)",
                numero, chunk[0]
            );
        }
    }

    println!("\nPrices after 10% discount:");
    for i in prices.iter_mut() {
        // Multiplicar por 0.9 reduce el precio un 10%
        *i *= 0.9;
        println!("${:.2}", i);
    }
}

