use std::io;
fn main() {
    // let v = vec![1, 2, 3];
    // for letra in v.iter() {
    //     println!("letra: {}", letra);
    // }
    // println!("v: {:?}", v);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let first = input.trim_end_matches(['\r', '\n']).to_string();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let second = input.trim_end_matches(['\r', '\n']).to_string();

    println!("{}", first)
}
