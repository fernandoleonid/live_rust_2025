struct RegistroVenda {
    mes: f64,
    valor: f64,
}

fn calcular_media(valores: &[f64]) -> f64 {
    let soma: f64 = valores.iter().sum::<f64>();
    let media: f64 = soma / valores.len() as f64;
    media
}

fn main() {
    let test = RegistroVenda  {
        mes: 4.0,
        valor: 100.99,
    };

    let valores = vec![1.0, 2.0, 3.0];
    let media = calcular_media(&valores);
    println!("Média: {}", media);    
    println!("Mes: {} venda: {}", test.mes, test.valor);
}
