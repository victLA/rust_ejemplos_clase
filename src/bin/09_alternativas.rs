#![allow(unused)]
fn main() {
    // let mut x = -5;
    // if x > 10 {
    //     x += 2;
    // } else {
    //     x -= 2;
    // }
    // println!("{x}");
    // let cosa = 30;
    // let result = if x >= 0 {
    //     print!("hola");
    //     let z = 27;
    //     x = 40;
    //     x
    // } else {
    //     print!("adios");
    //     let z = 27;
    //     x = 40;
    //     x
    // };
    // println!(" - {result} -");
    // let number = 5;
    // let result = if number < 10 { "Too small" } else { "100" };
    // println!("{}", result);
    // let valor = -4;
    // let mut txt = "";
    // if valor > 0 {
    //     txt = "positivo";
    // } else if valor == 0 {
    //     txt = "cero";
    // } else {
    //     txt = "negativo";
    // }
    // println!("El numero {valor} es {txt}");
    // let evaluation = true;
    // let num = match evaluation {
    //     true => 20,
    //     false => 40,
    // };

    // let nota = 6;
    // let resultado_final = match nota {
    //     0..5 => "suspenso",
    //     5..=7 => "aprobado",
    //     8..=10 => "destacado",
    //     _ => "Error en la nota",
    // };
    // println!("Resultado final: {}", resultado_final);

    // let n = -5;
    // match n {
    //     0 => println!("cero"),
    //     1 | 2 => println!("uno o dos"),            //patron de unión
    //     3..=9 => println!("entre 3 y 9"),          //patrón de rango
    //     x if x < 0 => println!("negativo: {}", x), // guarda
    //     _ => println!("otro"),                     // caso por defecto
    // }

    let number = 7;
    match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        value if value % 2 != 0 => println!("{value} is an odd number"),
        _ => unreachable!(), //esta opción no sería necesaria
    }

}
