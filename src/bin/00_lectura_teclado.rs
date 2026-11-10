
// use std::io::{self, Write};
// fn main() {
//     print!("Escribe tu teclado: ");
//     io::stdout().flush().unwrap(); // .flush() Vacía el búfer,  requiere el trait Write

//     let mut teclado = String::new();
//     io::stdin().read_line(&mut teclado).expect("Error al leer la entrada");

//     println!("Hola, {}", teclado.trim());
// }


use std::io;
fn main() {
    println!("Escribe tu nombre: ");    

    let mut teclado = String::new();
    io::stdin().read_line(&mut teclado).expect("Error al leer la entrada");
    println!("Hola, {}", teclado.trim());

    teclado.clear(); 

    println!("Escribe tu edad: ");
    io::stdin().read_line(&mut teclado).expect("Error al leer la entrada");
    println!("Tienes {} años.", teclado.trim());
}

