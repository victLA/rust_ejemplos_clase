struct Contador {
    valor: i32,
}
impl Contador {
    fn incrementar(&mut self) {
        self.valor += 1;
    }
    fn decrementar(&mut self) {
        self.valor -= 1;
    }
    fn obtener(&self) -> i32 {
        self.valor
    }
}
fn main() {
    // Al llamar a métodos mutables, la propia instancia debe declararse como mutable
    let mut contador = Contador { valor: 0 };

    contador.incrementar();
    contador.incrementar();
    println!("{}", contador.obtener()); // 2
    
    contador.decrementar();
    println!("{}", contador.obtener()); // 1
}
