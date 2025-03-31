fn main () {
    let arr = [10, 20, 30, 40];
    println!("Primeiro: {}", arr[0]);
    println!("Tamanho: {}", arr.len());
    for i in 0..arr.len() {
        println!("Elemento: {}", arr[i]);
    }
}