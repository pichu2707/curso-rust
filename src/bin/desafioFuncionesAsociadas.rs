// Crea una estructura Book con dos campos: title (String) y pages (u32).
//
// Añade un bloque de implementación para Book con una función asociada llamada new que:
//
// Acepte un parámetro title: String
// Devuelva un nuevo Book con el título proporcionado y pages establecido en 0
// Añade también un método llamado info que acepte &self y devuelva un String con el formato: "{title} has {pages} pages"
//
// Recibirás una entrada:
//
// El título del libro (String)
// Usa la función asociada Book::new para crear un libro con el título proporcionado y, después, imprime el resultado de llamar al método info.
//
// Formato de salida esperado:
//
// {title} has {pages} pages

use std::io;

// TODO: Define la struct Book con los campos title (String) y pages (u32)
struct Book {
    title: String,
    pages: u32,
}

// TODO: Añade un bloque impl para Book con:
// - Una función asociada `new` que toma un title y devuelve un Book con pages establecido en 0
// - Un método `info` que devuelve un String en el formato "{title} has {pages} pages"
impl Book {
    fn new(title: String) -> Book {
        Book { title, pages: 0 }
    }
    fn info(&self) -> String {
        format!("{} has {} pages", self.title, self.pages)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let title = input.trim().to_string();

    // TODO: Usa Book::new para crear un book con el title proporcionado
    let book = Book::new(title);
    // TODO: Imprime el resultado de llamar al método info
    println!("{}", book.info());
}
