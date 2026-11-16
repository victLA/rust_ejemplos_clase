use std::env;
fn main() {
    let cuantos = env::args().count();
    let mut args: Vec<String> = env::args().collect();

    println!("Datos recibidos");
    println!("Se ha recibido de consola {} argumentos", cuantos);
    args.remove(0);
    for elem in &args {
        println!("{}", elem)
    }
}
