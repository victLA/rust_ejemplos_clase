fn main() {
    let resultado = suma(10.0, 3.0);
    println!("{}", resultado);

    let resultado = dividir(10.0, 3.0);
    println!("{:.2}", resultado);

    let numeritos = vec![1, 2, 3, 4, 5, 6];
    let resultado = obtener_elemento(&numeritos, 8);
    match resultado {
        Some(n) => println!("el elementos del vector es {}", n),
        None => println!("no hay nada"),
    }

    fn suma(n1: f32, n2: f32) -> f32 {
        n1 + n2
    }

    match leer_cantidad() {
        Ok(valor) => println!("{}", valor),
        Err(_) => print!("Error!!"),
    }

    {
        let x = 3;
    }

    
}

fn leer_cantidad() -> Result<f64, std::num::ParseFloatError> {
    let mut entrada = String::new();
    std::io::stdin()
        .read_line(&mut entrada)
        .expect("Error al leer");

    entrada.trim().parse::<f64>()
}

fn dividir(n1: f32, n2: f32) -> f32 {
    if n2 == 0.0 {
        return 0.0;
    }
    if n1 <= 0.0 {
        return 0.0;
    }
    n1 / n2
}

fn factorial(n: i32) -> i32 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}

fn obtener_elemento(vec: &[i32], indice: usize) -> Option<i32> {
    if indice < vec.len() {
        Some(vec[indice])
    } else {
        None
    }
}

//fn main() {
//     let n = 4;
//     println!("{n}! = {}", factorial(n));
// }
