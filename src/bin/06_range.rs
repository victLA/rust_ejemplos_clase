fn main(){
    let r = 1..10;
    println!("{}", r.contains(&5)); // true
    println!("{}", r.contains(&10)); // false

    println!("----------------------------------");

    let dia_mes = 1..31;
    println!("{dia_mes:?}");
    
    println!("----------------------------------");
    let dia_mes = 1..=31;
    println!("{dia_mes:?}");
    for number in dia_mes {
        print!("{number} ");
    }
    

    println!("----------------------------------");
    let letra= 'a'..'f';
    for l in letra {
        println!("{l}");
    }
}   