#[derive(Debug)]


struct Punto<T > {
    x: T,
    y: T,
}

impl<T> Punto<T> {
    // implementacion de tipos genericos
    fn coordenadas(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
    fn establecer_x(&mut self, x: T) {
        self.x = x;
    }
 }

fn main() {
    let mut entero = Punto { x: 5, y: 4 };

    let flotante = Punto { x: 1.0, y: 4.6 };

    //let negativos: Punto<i8, i8> = Punto { x: 2, y: 6};

    entero.establecer_x(5);
    println!("{entero:?} y {flotante:?}");
    //println!("coordenadas: {:?}", entero.coordenadas());
     
}

