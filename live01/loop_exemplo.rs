fn main () {
    let mut contador = 0;

    loop {
        contador += 1;
        println!("Contador {}", contador);
        if contador == 4 {
            break;
        }
    }

}