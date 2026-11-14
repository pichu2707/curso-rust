// todo: define una struct pública product con dos campos:
// - name (string)
// - price (f64)
// recuerda hacer la struct y sus campos públicos
pub struct Product {
    pub name: String,
    pub price: f64,
}

// TODO: Implementa una función asociada llamada 'new' para Product
// - Debe tomar un name (String) y price (f64) como parámetros
// - Debe devolver una nueva instancia de Product
// - Recuerda hacer la función pública

impl Product {
    pub fn new(name: String, price: f64) -> Product {
        Product { name, price }
    }
}
