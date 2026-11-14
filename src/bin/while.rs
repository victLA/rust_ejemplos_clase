fn main() {
    // let mut n = 1;
    // while n <= 10 {
    //     println!("{n}");
    //     n += 1;
    //     if n == 5 {
    //         break;
    //     }
    // }

    // let mut pila = vec![1, 2, 3];
    // while let Some(top) = pila.pop() {
    //     println!("{}", top); // 3, 2, 1
    // }

    let mut segundos = 21;
    while segundos > 0 {       
        if segundos % 2 == 0 {
            println!("{segundos} segundos (número par), saltando 3 segundos...");
            segundos -= 3;
            continue;
        }       
        println!("{segundos} segundos para el despegue...");
        segundos -= 1;
    }
    println!("¡Despegue! 🚀");
    
}
