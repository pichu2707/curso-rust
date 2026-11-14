// TODO: Define una struct pública Locker con campos privados:
// - code: u32
// - contents: String
pub struct Locker {
    code: u32,
    contents: String,
}

impl Locker {
    // TODO: Implementa un constructor público 'new'
    pub fn new(code: u32, contents: String) -> Self {
        Self { code, contents }
    }
    // Toma el code inicial (u32) y contents (String)
    // Devuelve una nueva instancia de Locker

    // TODO: Implementa un método público 'set_code'
    // Toma un nuevo code (u32) y actualiza el code del locker
    pub fn set_code(&mut self, code: u32) {
        self.code = code;
    }

    // TODO: Implementa un método público 'get_contents'
    // Toma un code intentado (u32)
    pub fn get_contents(&self, code: u32) -> String {
        if self.code == code {
            self.contents.clone()
        } else {
            String::from("Access denied")
        }
    }
    // Returns String: the contents if code matches, or "Access denied" if not
}
