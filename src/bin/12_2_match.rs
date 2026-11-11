fn main() {
    // desectructuración
    println!("------------------- tupla-------------------------");
    // Un ejemplo de cómo usar `match` para destructurar una tupla
    let triple = (0, -2, 3);

    println!("Cuentáme sobre {:?}", triple);
    // Match can be used to destructure a tuple
    match triple {
        // Desectructura el primer valor y enlaza el resto a `y` y `z`
        (0, y, z) => println!("Primero es `0`, `y` es {:?}, y `z` es {:?}", y, z),
        (1, ..) => println!("Primero es `1` y el resto no importa"),
        (.., 2) => println!("El último es `2` y el resto no importa"),
        (3, .., 4) => println!("Primero es `3`, el último es `4`, y el resto no importa"),
        // `..` se puede usar para ignorar el resto de la tupla
        _ => println!("No importa cuáles sean"),
    }
    println!("---------------- array----------------------------");
    // Prueba a cambiar los valores del array, ¡o conviértelo en un slice!
    let array = [1, -2, 6];

    match array {
        // Asigna el segundo y el tercer elemento a las variables respectivas
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // Los valores individuales se pueden ignorar con _
        [1, _, third] => println!("array[0] = 1, array[2] = {} y array[1] fue ignorado", third),

        // También puedes asignar algunos y ignorar el resto
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} y todos los demás fueron ignorados",
            second
        ),
        // El siguiente código no compilaría
        // [-1, second] => ...

        // O almacenarlos en otro array/slice (el tipo depende de
        // el del valor con el que se está comparando)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} y los demás elementos eran {:?}",
            second, tail
        ),

        // Combinando estos patrones, podemos, por ejemplo, asignar el primer y
        // el último valor, y almacenar el resto en un único array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }

    println!("---------------- referencias ----------------------------");
    // Asigna una referencia de tipo `i32`. El `&` indica que
    // se está asignando una referencia.
    let reference = &4;

    match reference {
        // Si `reference` se compara con el patrón `&val`, resulta
        // en una comparación como:
        // `&i32`
        // `&val`
        // ^ Vemos que si se eliminan los `&` que coinciden, entonces el `i32`
        // debería asignarse a `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // Para evitar el `&`, desreferencia antes de comparar.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // ¿Y si no empiezas con una referencia? `reference` era un `&`
    // porque el lado derecho ya era una referencia. Esto no es
    // una referencia porque el lado derecho no lo es.
    let _not_a_reference = 3;

    // Rust proporciona `ref` exactamente para este propósito. Modifica
    // la asignación para que se cree una referencia al elemento; esa
    // referencia es la que se asigna.
    let ref _is_a_reference = 3;

    // Por tanto, al definir 2 valores sin referencias, se pueden
    // obtener referencias mediante `ref` y `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Usa la palabra clave `ref` para crear una referencia.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Usa `ref mut` de forma similar.
    match mut_value {
        ref mut m => {
            // Tenemos una referencia. Hay que desreferenciarla antes de
            // poder sumarle algo.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }

    println!("---------------- structs ----------------------------");
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Prueba a cambiar los valores del struct para ver qué ocurre
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // puedes desestructurar structs y renombrar las variables,
        // el orden no importa
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // y también puedes ignorar algunas variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // esto dará un error: el patrón no menciona el campo `x`
        //Foo { y } => println!("y = {}", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };

    // No necesitas un bloque match para desestructurar structs:
    let Foo { x: x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");

    // La desestructuración también funciona con structs anidados:
    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar {
        foo: Foo {
            x: nested_x,
            y: nested_y,
        },
    } = bar;
}
