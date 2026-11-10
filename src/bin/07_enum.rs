#![allow(dead_code)]
#![allow(unused)]
#[derive(Debug)]
enum Direccion {
    Izquierda,
    Derecha,
}

#[derive(Debug)]
enum MovimientoJugador {
    Pasar,                              // Variante simple
    Correr(Direccion),                  // Variante de tupla
    Teletransportar { x: u32, y: u32 }, // Variante de struct
}
enum Estaciones {
    Primavera,
    Verano,
    Otoño,
    Invierno,
}
fn main() {
    let m: MovimientoJugador = MovimientoJugador::Correr(Direccion::Izquierda);
    let mut m2 = MovimientoJugador::Correr(Direccion::Izquierda);
    let m2 = MovimientoJugador::Teletransportar { x: 3, y: 60 };

    println!("En este turno: {:?}", m);

    let mover = Direccion::Derecha;

    let estamos_en = Estaciones::Otoño;
}
