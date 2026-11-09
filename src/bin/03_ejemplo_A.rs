fn doblar(x: i32) -> i32 {
    x * 2
}


fn main() {
    let n = 5;
    
    let resultado = doblar(n);

    println!("{n}");          // ✅ funciona, n sigue siendo 5
    println!("{resultado}");  // 10
}

//=====================

fn consumir(s:&mut String) {
    s.push_str(" adios");
    println!("{s}");
}   // s se libera aquí

fn main() {
    let mut texto = String::from("hola");
    consumir(&mut texto);
    println!("{texto}");   
}