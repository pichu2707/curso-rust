// TODO: Define una struct pública UserProfile con campos privados:
// - username: String
// - age: u32
pub struct UserProfile {
    username: String,
    age: u32,
}

// TODO: Implementa lo siguiente para UserProfile:
// - pub fn new(username: String, age: u32) -> Self
// - pub fn username(&self) -> &String (getter para username)
// - pub fn age(&self) -> u32 (getter para age)
impl UserProfile {
    pub fn new(username: String, age: u32) -> Self {
        UserProfile{username, age}
    }
    pub fn username(&self) -> &String {
        &self.username
    }
    pub fn age(&self) -> u32 {
        self.age
    }
}
