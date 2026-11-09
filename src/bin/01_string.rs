#![allow(unused)]
fn main() {
    //creación vacio, texto
    // let mut nombre = String::from("Jorgito");
    // nombre.push_str(" Donnal");
    // println!("El nombre es: {nombre}");
    // let mut datos = String::new();
    // let saludo = "Hola mundo";
    // datos.push_str("Buenass");
    // let a = "hola".to_owned();
    // let b = "hola".to_string();
    // let c = String::from("hola");
    // let d: String = "hola".into();
    // let mut s1 = String::new(); // vacío
    // let s2 = String::from("hola");
    // let s3 = "hola".to_string(); //pasa a String cualquier tipo
    // let num = 42.to_string(); // número -> String
    // let s4 = "hola".to_owned(); //pasa str a String , guarda en el head, ahora es mutable y dueño
    // let s5 = String::with_capacity(50); // reserva capacidad

    //conversión
    // let num = 42.to_string(); // número -> String
    // let n: i32 = "42".parse().expect("error"); // String -> número

    //Modificación (requiere mut), insert, remove, clear
    // let mut s = String::from("hola");
    // println!();
    // s.push('!'); // añade un char
    // s.push_str(" mundo"); // añade un &str
    // s.insert(0, '¡'); // inserta char en posición (en bytes)
    // s.insert_str(2, "ey, "); // inserta &str
    // println!("{}", s);
    // s.pop(); // quita y devuelve el último char (Option<char>)
    // s.remove(0); // quita el char en un índice
    // //s.truncate(5); // corta a N bytes
    // s.retain(|c| c != 'a'); // conserva solo los chars que cumplan
    // println!("{}", s);
    // s.clear();

    //Concatenación
    // let mut a = String::from("Hola");
    // let b = String::from(" mundo");
    // let c = a + &b; // `a` se mueve, ya no se puede usar
    // let d = format!("{}{}", c, "!"); // no mueve nada
    // let mut e = String::new();
    // e += "texto"; // equivale a push_str
    // println!("{}", c);

    // Consulta
    // let s = "¡Hola mundo!";
    // s.len(); // longitud en BYTES (no chars)
    // s.chars().count(); // número de caracteres
    // println!("La longitud en caracteres es: {}", s.chars().count());
    // s.is_empty();
    // s.contains("mundo"); // bool
    // s.starts_with("Hol");
    // s.ends_with("do");
    // s.find("mundo");
    // s.rfind('o'); // desde el final
    // s.matches("o").count(); // cuántas veces aparece
    // let s = " Hola Mundo ";
    // let result = s.replace("Hola", "Adiós"); // reemplaza todas
    // "hola.rs".strip_suffix(".rs"); // Option<&str>
    // if let Some(cosa) = "hola.rs".strip_suffix(".xls") {
    //     println!("{cosa}");
    // }

    //División e iteración
    // let s = "uno;dos;tres";
    // let partes: Vec<&str> = s.split(';').collect(); //también con String
    // println!("{partes:?}");
    // for i in partes {
    //     println!("{i}");
    // }
    // s.splitn(2, ','); // máximo 2 partes
    // s.split_once(','); // Some(("uno", "dos, tres"))
    // "a b c".split_whitespace(); // ignora espacios múltiples

    //recorrido
    // for c in "hola".chars() {} // por caracteres
    // for b in "hola".bytes() {} // por bytes
    // for (i, c) in "hola".char_indices() {
    //     println!("index {i} - valor {c}");

    let s = "Hola mundo";
    let hola = &s[0..4]; // "Hola"
    let resto = &s[5..]; // "mundo"

    let seguro = s.get(0..20); // Option<&str>, no paniquea
    println!("{seguro:?}");

    //Formato
    let nombre = "Ana";
    let edad = 30;
    format!("{} tiene {} años", nombre, edad);
    format!("{nombre} tiene {edad} años"); // captura inline
    println!("{}", format!("{:>10}", nombre)); // alinea a la derecha
    println!("{}", format!("{:<10}xx", nombre));

    format!("{:05}", 42); // "00042"
    format!("{:.2}", 3.14159); // "3.14"
    format!("{:?}", nombre); // Debug: "\"Ana\""
}
