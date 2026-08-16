// Recibirás cuatro entradas: el título de un libro, el nombre de un autor, un año de publicación (como un entero) y un número de páginas (como un entero). Define una estructura Book con cuatro campos: title de tipo String, author de tipo String, year de tipo i32 y pages de tipo i32. Crea una instancia de esta estructura utilizando los valores de entrada e imprime la información del libro.
//
// Requisitos:
//
// Definir una estructura Book con los campos: title: String, author: String, year: i32 y pages: i32
// Leer la primera entrada como el título del libro
// Leer la segunda entrada como el nombre del autor
// Leer la tercera entrada y convertirla a i32 para el año de publicación
// Leer la cuarta entrada y convertirla a i32 para el número de páginas
// Crear una instancia de la estructura Book con estos valores
// Imprimir la información del libro en el formato exacto que se muestra a continuación
// Entrada:
//
// Primera línea: Título del libro (por ejemplo, The Rust Programming Language)
// Segunda línea: Nombre del autor (por ejemplo, Steve Klabnik)
// Tercera línea: Año de publicación como un entero (por ejemplo, 2018)
// Cuarta línea: Número de páginas como un entero (por ejemplo, 552)
// Salida:
//
// Primera línea: Book: [title]
// Segunda línea: Author: [author]
// Tercera línea: Year: [year]
// Cuarta línea: Pages: [pages]

use std::io;

// TODO: Define la estructura Book aquí con los campos: title, author, year y pages

struct Book {
    title: String,
    author: String,
    year: i32,
    pages: i32,
}

fn main() {
    // Leer el título del libro
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");
    let title = title.trim().to_string();

    // Leer el nombre del autor
    let mut author = String::new();
    io::stdin()
        .read_line(&mut author)
        .expect("Failed to read line");
    let author = author.trim().to_string();

    // Leer el año de publicación
    let mut year_input = String::new();
    io::stdin()
        .read_line(&mut year_input)
        .expect("Failed to read line");
    let year: i32 = year_input.trim().parse().expect("Failed to parse year");

    // Leer el número de páginas
    let mut pages_input = String::new();
    io::stdin()
        .read_line(&mut pages_input)
        .expect("Failed to read line");
    let pages: i32 = pages_input.trim().parse().expect("Failed to parse pages");

    // TODO: Crear una instancia de la estructura Book usando los valores de entrada
    let book = Book {
        title,
        author,
        year,
        pages,
    };
    // TODO: Imprimir la información del libro en el formato requerido
    println!("Book: {}", book.title);
    println!("Author: {}", book.author);
    println!("Year: {}", book.year);
    println!("Pages: {}", book.pages);
}
