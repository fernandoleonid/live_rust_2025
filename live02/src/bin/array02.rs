fn main() {
    let nomes = ["Ana", "Bruno", "Clara", "Diego", "Eva"];

    println!("Nomes originais: {:?}", nomes);

    for i in 0..nomes.len() {
        println!("Cliente {}: {}", i+1, nomes[i]);
    }
}