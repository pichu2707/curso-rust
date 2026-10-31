Recibirás dos entradas. La primera entrada es un número que representa un precio base. La segunda entrada es un número que representa un porcentaje de descuento. Crea un cierre que capture el porcentaje de descuento del entorno y calcule el precio final después de aplicar el descuento. Llama al cierre con el precio base e imprime el precio final.

Requisitos:

Lee la primera entrada (precio base) y elimina los espacios en blanco
Analízala como f64
Lee la segunda entrada (porcentaje de descuento) y elimina los espacios en blanco
Analízala como f64
Crea un cierre que acepte un parámetro (el precio base) y capture el porcentaje de descuento del entorno circundante
El cierre debe calcular el precio final usando la fórmula: base_price - (base_price * discount / 100.0)
Llama al cierre con el precio base
Imprime el precio final
Entrada:

Primera línea: Un precio base (por ejemplo, 100.0)
Segunda línea: Un porcentaje de descuento (por ejemplo, 20.0)
Salida:

El precio final después de aplicar el descuento

use std::io;

fn main() {
    // Leer el precio base
    let mut base_price_input = String::new();
    io::stdin().read_line(&mut base_price_input).expect("Failed to read line");
    let base_price: f64 = base_price_input.trim().parse().expect("Invalid number");
    
    // Leer el porcentaje de descuento
    let mut discount_input = String::new();
    io::stdin().read_line(&mut discount_input).expect("Failed to read line");
    let discount: f64 = discount_input.trim().parse().expect("Invalid number");
    
    // TODO: Crear un closure que capture el descuento y calcule el precio final
    let value_discount = | base_price: f64 | base_price - ((discount/100.00) * base_price);
    // TODO: Llamar al closure con el precio base
   let final_price = value_discount(base_price);
    // Imprimir el precio final
    println!("{}", final_price);
}
