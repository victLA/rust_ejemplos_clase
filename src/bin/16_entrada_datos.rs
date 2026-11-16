use std::io;

fn main() {
    println!("read_line");
    // opcion 1
    println!("1 Entrada de cualquier caracter");
    let mut cadena = String::new(); // cadena entrada
    io::stdin()
        .read_line(&mut cadena)
        .expect("Fallo de Entrada");    
    println!("Lectura String {}", cadena);

    // opcion 2
    println!("2 Recibir solo valor numerico");
    let mut mi_int: i32; // Solo Recibe numero entero
    loop {
        let mut var_check: bool = true;
        println!("Ingrese un numero entero:");
        let mut numero = String::new();
        io::stdin()
            .read_line(&mut numero)
            .expect("Fallo de Entrada");
        // chequear si recibimos unn int
        let entero = match numero.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("[ERR] No es número entero!!");
                var_check = false;
                0
            }
        };
        // confirmar si es integer para salir
        mi_int = entero;
        if var_check  {
            break;
        };
    }
    println!("Valor entero = {:?}", mi_int); // salida entero
}
