// TODO: Define una struct pública Pet con campos privados:
// - name: String
// - energy: u32
// - hunger: u32
pub struct Pet {
    name: String,
    energy: u32,
    hunger: u32,
}

// TODO: Implementa lo siguiente para Pet:
// - Una función asociada pública `new` que toma un name (String)
//   y devuelve un Pet con energy = 100 y hunger = 0
// - Un método público `name` que devuelve una referencia al name de la mascota (&String)
impl Pet {
    pub fn new(name: String) -> Self {
        Self {
            name,
            energy: 100,
            hunger: 0,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
