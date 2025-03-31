struct Produto {
    nome: String,
    preco: f64,
}
impl Produto {
    fn novo(nome: String, preco: f64) -> Produto {
        Produto {nome, preco}
    }
    fn mostrar_info(&self) {
        println!("Produto: {}, Preço: R$ {}", self.nome, self.preco)
    }

    fn aplicar_desconto(&mut self, desconto: f64) {
        self.preco = self.preco - (self.preco * desconto);
    }
}

item1.mostrar_info();
item2.mostrar_info();
fn main () {
    let mut item1 = Produto::novo(String::from("Teclado"), 100.00);
    let mut item2 = Produto::novo(String::from("Mouse"), 88.99);

    item1.mostrar_info();
    item2.mostrar_info();

    item1.aplicar_desconto(0.10);
    item2.aplicar_desconto(0.50);

    item1.mostrar_info();
    item2.mostrar_info();

}