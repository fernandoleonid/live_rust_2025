fn main() {
    // CRUD
    // CREATE
    let mut idades: Vec<i32> = Vec::new();
    idades.push(15);
    idades.push(23);
    idades.push(45);
    println!("Após adicionar: {:?}", idades);

    // READ
    let indice = 1;
    println!("A idade {}: {}", indice, idades[indice]);

    // UPDATE
    idades[indice] = 100;
    println!("A idade {} depois: {}", indice, idades[indice]);

    // DELETE
    idades.remove(2);
    println!("Após deletar: {:?}", idades);
}