fn main() {
    let mut v_nuevo = Vec::new(); //hay que indicar tipo si está vácio     
    v_nuevo.push(60);

    let mut v = vec![1, 2, 3];

    // let mut numeros = vec![1, 2, 3, 4, 5];
    // numeros.push(6);
    // numeros.push(7);
    // numeros.push(8);
    // numeros[0] = 100;
    // if let Some(valor) = numeros.pop() {
    //     println!("{valor}");
    // } else {
    //     println!("None");
    // }

    // numeros.insert(3, 200);
    // numeros.extend([9, 8, 7, 6]);
    // //numeros.extend(v);
    // numeros.append(&mut v);
    // println!("El vector es: {:?}",

    // println!("El vector es: {:?}", numeros);

    for n in &v {
        println!("{}", n);
    } // solo leer


    for n in v.iter_mut() {
        *n *= 2;
    } // modificar cada elemento

    println!("{v:?}");

    // let mut nombres = Vec::new(); //no conoce el tipo y usa un genérico Vec<T>
    // nombres.push("Juanito");
    // nombres.push("Jorgito");
    // nombres.push("Jaimito");
    // let first = nombres[0];
    // println!("Los nombres son: {:?} y el primero es {}", nombres, first);
    // for n in &nombres{
    //     println!("{n}");
    // }
    // println!("{}", nombres[1]);

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    //las originales se mueven al vector y ya no se pueden usar
    let pizza_toppings = vec![pepperoni, mushroom, sausage];

    // println!("{}",pepperoni); // Error no está

    //no puedo mover la cadena original porque se mueve y el vector quedaría con propiedad parcial
    //tiene que ser con referencia
    //let pizza_slice = pizza_toppings[1]; //Error!!

    let pizza_slice = &pizza_toppings[1]; //correcto
    let pizza_slice = &pizza_toppings;
}
