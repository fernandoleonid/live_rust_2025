fn main(){
    // CREATE
    let mut arr = [12, 21, 39, 43, 56 ];
    println!("Array inicial: {:?}", arr);

    // READ
    println!("Elemento no índice {}: {}", 2, arr[2]);

    // UPDATE
    arr[1] = 200;
    println!("Após atualizar: {:?}", arr);

    // DELETE - Simula remover, mas não remove
    arr[3] = 0;
    println!("Após deletação simulada: {:?}", arr);

    // Precorrer todos os elementos
    for i in 0..arr.len() {
        println!("Produto {} = {}", i, arr[i]);
    }
}