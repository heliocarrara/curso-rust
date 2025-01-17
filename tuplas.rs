fn main(){
    let tupla :(&str, i32, f64) = ("Olá, mundo!", 100, 3.14);

    println!("Primeiro elemento da tupla: {}", tupla.0);
    println!("Segundo elemento da tupla: {}", tupla.1);
    println!("Terceiro elemento da tupla: {}", tupla.2);
}