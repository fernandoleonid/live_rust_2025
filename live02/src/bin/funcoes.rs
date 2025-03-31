fn mensagem () {
    println!("FECAF");
}

fn somar(a: i32, b: i32) {
    println!("A soma dos número é: {}", a + b)
}

fn multiplicar(x: i32, y: i32) -> i32 {   
    x * y
}

fn main() {
    mensagem();
    somar(10, 15);
    let resultado = multiplicar(5, 10);
    println!("Multiplicação: {}", resultado);
}