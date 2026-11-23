// TODO: Define una estructura pública Profile con:
// - username: String
// - nickname: Option<String>

pub struct Profile {
    // TODO: Añade los campos aquí
    username: String,
    nickname: Option<String>,
}

impl Profile {
    // TODO: Implementa el constructor new
    // Recibe un username (String) y un nickname opcional (Option<String>)
    pub fn new(username: String, nickname: Option<String>) -> Self {
        Self { username, nickname }
    }

    // TODO: Implementa el método display_name
    // Devuelve nickname si existe o username en caso contrario
    // Pista: Usa unwrap_or o unwrap_or_else
    pub fn display_name(&self) -> String {
        self.nickname.clone().unwrap_or(self.username.clone())
    }

    // TODO: Implementa el método formatted_nickname
    // Devuelve Option<String> con el nickname entre corchetes, como "[CoolNick]"
    // Devuelve None si no hay nickname
    // Pista: usa map para transformar el valor de Option
    pub fn formatted_nickname(&self) -> Option<String> {
        self.nickname.as_ref().map(|nick| format!("[{}]", nick))
    }
}
