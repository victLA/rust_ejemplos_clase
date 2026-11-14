fn main() {
    // let mut n = 0;
    // loop {
    //     n += 2;
    //     println!("{n}");
    //     if n == 10 {
    //         break;
    //     }
    // }

    // let mut contador = 0;
    // let resultado = loop {
    //     contador += 1;
    //     println!("Valor: {contador}");
    //     if contador == 5 {
    //         break "llegamos a 5"; //no es necesario el mensaje
    //     }
    // };
    //println!("El loop devolvió {}", resultado); //imprime “llegamos a 5”

    // let mut segundos = 11;
    // loop {
    //      segundos -= 1;
    //     if segundos == 5 || segundos == 3{
    //         continue;
    //     }
    //     if segundos == 0 {
    //         println!("¡Despegue! 🚀");
    //         break;
    //     }
    //     println!("{segundos} segundos para el despegue...");

    // }

    // 'externo: loop {
    //     println!("loop1");
    //     'segundo: loop {
    //         println!("loop2");
    //         loop {
    //             println!("loop3");
    //             break 'segundo;
    //         }
    //     }
    // }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    println!("fuera del loop");

}
