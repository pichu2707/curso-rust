// TODO: Define una struct pública Thermostat con un campo temperature privado (i32)

pub struct Thermostat {
    temperature: i32,
}

impl Thermostat {
    // TODO: Implementa un constructor 'new' que tome una temperatura inicial
    pub fn new(temperature: i32) -> Self {
        // TODO: Crea y devuelve un nuevo Thermostat
        Self { temperature }
    }

    // TODO: Implementa un getter 'temperature' que devuelva la temperatura actual
    pub fn temperature(&self) -> i32 {
        // TODO: Devuelve la temperature
        self.temperature
    }

       // Solo acepta valores entre 10 y 30 (inclusive)
    // Si el valor está fuera de este rango, mantén la temperature sin cambios
    pub fn set_temperature(&mut self, temp: i32) {
        // TODO: Valida y establece la temperature
        if temp >= 10 && temp <= 30 {
            self.temperature = temp;
        }
    }
}

