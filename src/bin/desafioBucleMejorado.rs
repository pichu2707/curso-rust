fn main() {
    let mut prices = [2.75, 1.50, 5.00, 3.5, 4.1, 2.25, 7.9];

    println!("Original Prices:");
    for (index, value) in prices.iter().enumerate() {
        let number = index + 1;
        println!("Item {}: ${}", number, value);
    }
    // Expected: "Item 1: $2.99" etc.

    println!("\nBundle Deals:");
    // TODO: Use chunks to print pairs of prices and their sums
    // Expected: "Bundle 1: $2.99 + $1.50 = $4.49" etc.

    // TODO: Use iter_mut to apply 10% discount to all prices

    println!("\nPrices after 10% discount:");
    // TODO: Print final prices after discount
    // Expected: "$2.69" etc.
}
