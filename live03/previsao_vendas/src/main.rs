struct RegistroVenda {
    mes: f64,
    valor: f64,
}

fn calcular_media (valores: &[f64]) -> f64 {

    let soma: f64 = valores.iter().sum::<f64>();
    let media: f64 = soma / valores.len() as f64;

    media
}

fn calcular_inclinacao(dados: &[RegistroVenda]) -> f64 {

   
    let meses: Vec<f64> = dados.iter().map(|d| d.mes).collect();
    let valores: Vec<f64> = dados.iter().map(|d| d.valor).collect();
    
    let media_meses = calcular_media(&meses);
    let media_valores = calcular_media(&valores);

    let numerador: f64 = dados.iter()
        .map(|d| (d.mes - media_meses) * (d.valor - media_valores))
        .sum();
    let denominador: f64 = dados.iter()
        .map(|d| (d.mes - media_meses).powi(2))
        .sum();
    numerador / denominador
}

fn calcular_intercepto(dados: &[RegistroVenda], inclinacao: f64) -> f64 {
    let media_meses = calcular_media(&dados.iter()
                .map(|d| d.mes).collect::<Vec<f64>>());
    let media_valores = calcular_media(&dados.iter()
                .map(|d| d.valor).collect::<Vec<f64>>());
    media_valores - (inclinacao * media_meses)
}

fn prever_valor(intercepto: f64, inclinacao: f64, mes: f64) -> f64 {
    intercepto + (inclinacao * mes)
}

fn main() {

    let dados_vendas = vec![
        RegistroVenda { mes: 1.0, valor: 100.0 },
        RegistroVenda { mes: 2.0, valor: 120.0 },
        RegistroVenda { mes: 3.0, valor: 140.0 },
    ];
    let inclinacao = calcular_inclinacao(&dados_vendas);
    let intercepto = calcular_intercepto(&dados_vendas, inclinacao);
    let previsao = prever_valor(intercepto, inclinacao, 6.0);

    println!("Inclinação: {:.2}, Intercepto: {:.2}", inclinacao, intercepto);
    println!("Previsão para mês 6: {:.2}", previsao);

}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_calcular_media() {
        let valores = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let resultado = calcular_media(&valores);
        assert_eq!(resultado, 3.0)
    }

    #[test]
    fn test_calcular_media_iguais() {
        let valores = vec![9.0, 9.0, 9.0, 9.0, 9.0];
        let resultado = calcular_media(&valores);
        assert_eq!(resultado, 9.0)
    }

    #[test]
    fn test_calcular_media_unico_valor() {
        let valores = vec![5.0];
        let resultado = calcular_media(&valores);
        assert_eq!(resultado, 5.0)
    }

    #[test]
    fn test_calcular_media_negativos() {
        let valores = vec![-2.0, -4.0, -6.0];
        let resultado = calcular_media(&valores);
        assert_eq!(resultado, -4.0)
    }

    #[test]
    fn test_calcular_media_negativos_positivos() {
        let valores = vec![-2.0, 4.0];
        let resultado = calcular_media(&valores);
        assert_eq!(resultado, 1.0)
    }
}
