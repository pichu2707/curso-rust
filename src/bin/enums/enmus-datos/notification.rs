// TODO: Deriva el trait Debug para que el enum se pueda imprimir con {:?}

// TODO: Define un enum público Notification con tres variantes:
// - Alert: contiene un único mensaje String (sintaxis de tupla)
// - Reminder: contiene campos con nombre: title (String) y minutes (u32)
// - Dismiss: no contiene datos
#[derive(Debug)]
pub enum Notification {
    Alert(String),
    Reminder { title: String, minutes: u32 },
    Dismiss,
}
