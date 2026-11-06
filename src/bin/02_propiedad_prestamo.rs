fn main() {
    // let mi_valor_pila = 2; //dato guardado en stack
    // let mi_referencia_entero = &mi_valor_pila;

    // println!("{} - {}", mi_referencia_entero, mi_valor_pila); //se puede quitar el *


    // let mi_valor_heap = String::from("Hola, caracola");
    // let mi_referencia_heap = &mi_valor_heap;
    // let otra_ref_heap = &mi_valor_heap;

    // println!("{} - {} - {}", mi_referencia_heap, mi_valor_heap,otra_ref_heap); //se puede quitar el *

//     let coche = String::from("Rojo");
// let ref1 = &coche;
// let ref2 = &coche;
// println!("{ref1} y {ref2} y {}", &coche);


//Una referencia mutable (NO copy -> prestamos de propiedad )

let mut cafe = String::from("Moca");

let a = &mut cafe; 

println!("{a}");

let b = a; //cambio de propiedad, a ya no existe

//println!("{cafe}");

println!("{b}");

//el préstamo se devuelve después del último uso de variable
 println!("{cafe}");




}