fn main() {
    let mut x = 5;

    if x > 10 {
        x += 2;
    } else {
        x -= 2;
    }
    println!("{x}");
    let cosa = 30;

    let result = if x >= 0 {
        print!("hola");
        let z = 27;
        x = 40;
        x
    } else {
        0
    };

    println!("{result}");

    let number = 5;
    let result = if number < 10 { "Too small" } else { "100" };
    println!("{}", result);

    let valor = -4;
    let mut txt = "";

    if valor > 0 {
        txt = "positivo";
    }else if valor == 0 {
        txt = "cero";
    }else {
        txt = "negativo";
    }

    println!("El numero {valor} es {txt}");

let evaluation = true;
   let num = match evaluation {
        true => 20,
        false => 40
    };

    let nota = 6;
    let resultado_final = match nota {
        0 => "suspenso",
        5 => "aprobado",
        7 => "notable"
        
    };
}
