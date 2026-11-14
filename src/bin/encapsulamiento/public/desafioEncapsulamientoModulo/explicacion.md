Construyamos un catálogo de productos sencillo organizando tu código en dos archivos y haciendo públicos los elementos adecuados para que puedan usarse desde tu programa principal.

Crearás dos archivos:

product.rs: Define una estructura Product con dos campos: name (String) y price (f64). Añade una función asociada llamada new que tome un nombre y un precio, y devuelva un nuevo Product. Recuerda marcar la estructura, sus campos y el método como públicos para que se pueda acceder a ellos desde fuera del módulo.
main.rs: Declara el módulo de productos, introduce Product en el ámbito con una instrucción use, después crea un producto e imprime sus detalles.
Piensa detenidamente dónde necesitas la palabra clave pub: la estructura en sí debe ser visible, los campos deben ser accesibles para su lectura y el método constructor debe poder llamarse desde main.rs.

Tu salida debe seguir este formato exacto:

`Product: {name}, Price: ${price}`
Por ejemplo, si creas un producto con el nombre `"Laptop"` y el precio `999.99`, la salida sería:

`Product: Laptop, Price: $999.99`

Recibirás dos entradas: el nombre del producto y el precio.
