#[derive(Debug)]
#[allow(dead_code)]

struct Persona {
    nombre: String,
    edad: u8,
}

struct Pecera{
    tamaño: String,
    color: String,
    capacidad: f64,   
}



fn main() {
    let mut p = Persona {
        nombre: String::from("Ana"),
        edad: 30,
    };
    p.edad = 31;
    p.nombre = String::from("Jorgito");

    let  mut p2= Persona { 
        nombre:  String::from("Juanito"),
        edad: 24,
    };

    let mi_pecera = Pecera {
        tamaño: String::from("Grande"),
        color: String::from("azul"),
        capacidad: 34.5,
    };
        let antonio_pecera = Pecera {
        tamaño: String::from("Grande"),
        color: String::from("azul"),
        capacidad: 34.5,
    };

}
