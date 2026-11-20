struct DuracionCorta(i32, i32); // Horas, minutos
struct DuracionLarga(i32, i32); // Años, meses

struct Turno(i32, String);

fn main() {
    let turno_trabajo = DuracionCorta(8, 30);

    //let era = DuracionLarga(5, 3);

    ir_a_trabajar(&turno_trabajo);
    // aceptar_tupla(era);

    //let turno_trabajo = (8, 0); //esto es una tupla, no un struct

    // let era = (5, 3);
    ir_a_trabajar(&turno_trabajo);
    // ir_a_trabajar(era);

    let x = 10;
    let y = x;


}
fn ir_a_trabajar(duracion: &DuracionCorta) {
    println!(
        "Tiempo transcurrido: {} horas {} minutos",
        duracion.0, duracion.1
    );
}



