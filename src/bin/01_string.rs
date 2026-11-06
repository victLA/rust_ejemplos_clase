#![allow(unused)]
fn main() {
    let mut nombre = String::from("Jorgito");
    nombre.push_str(" Donnal");
    println!("El nombre es: {nombre}");

    let mut datos = String::new();
    let  saludo = "Hola mundo";

    datos.push_str("Buenass");

let a = "hola".to_owned();
let b = "hola".to_string();
let c = String::from("hola");
let d: String = "hola".into();

let mut s1 = String::new(); // vacío
let s2 = String::from("hola");
let s3 = "hola".to_string(); //pasa a String cualquier tipo
let num = 42.to_string(); // número -> String
let s4 = "hola".to_owned(); //pasa str a String , guarda en el head, ahora es mutable y dueño
let s5 = String::with_capacity(50); // reserva capacidad

//conversión
let num = 42.to_string(); // número -> String
let n: i32 = "42".parse().expect("error"); // String -> número

//Modificación (requiere mut)
let mut s = String::from("hola");
println!();
s.push('!'); // añade un char
s.push_str(" mundo"); // añade un &str
s.insert(0, '¡'); // inserta char en posición (en bytes)
s.insert_str(2, "ey, "); // inserta &str 

 println!("{}", s);
 s.pop(); // quita y devuelve el último char (Option<char>)
 s.remove(0); // quita el char en un índice
 //s.truncate(5); // corta a N bytes 
 s.retain(|c| c != 'a'); // conserva solo los chars que cumplan
  println!("{}", s);
s.clear();

//Concatenación
let a = String::from("Hola");
let b = String::from(" mundo");

let c = a + &b; // `a` se mueve, ya no se puede usar
// let d = format!("{}{}", c, "!"); // no mueve nada
// let mut e = String::new();
// e += "texto"; // equivale a push_str

println!("{}", c);




}


