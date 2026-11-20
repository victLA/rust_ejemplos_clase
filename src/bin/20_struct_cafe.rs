#![allow(unused)]

use std::vec;
#[derive(Debug)]
struct Cafe {
    precio: f64,
    nombre: String,
    es_caliente: bool,
    aroma: String,
}
impl Cafe {
    fn inf_cafe(&self) {
        println!(
            " El café {}, cuesta {} y es {}",
            self.nombre, self.precio, self.aroma
        );
    }
    fn subida_precio(&mut self, subida: f64) {
        //subior 20%
        let subida_precio = self.precio * subida;

        self.precio = self.precio + subida_precio;
    }
    fn new(nombre: String, aroma: String, precio: f64, es_caliente: bool) -> Self {
        Self {
            nombre,
            precio,
            es_caliente,
            aroma,
        } //no tiene ; y será lo que devuelve la función
    }
    fn olerrr() {
        println!("ummmmm");
    }
}

fn main() {
    let mut moca = Cafe {
        precio: 5.70,
        aroma: String::from("fuerte"),
        nombre: String::from("Moca"),
        es_caliente: true,
    };
    let cafe_hielo = Cafe {
        nombre: String::from("Café con hielo"),
        precio: 4.50,
        es_caliente: false,
        aroma: String::from("citrico"),
    };

    moca.inf_cafe();
    moca.subida_precio(0.20);

    let mut cafe_mañanas = Cafe::new(String::from("olé"), String::from("rico"), 4.8, false);
    cafe_mañanas.inf_cafe();
    cafe_mañanas.subida_precio(0.10);
    
}
