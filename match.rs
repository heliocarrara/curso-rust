fn main(){
    let fruta: &str = "Goiabada";

    match fruta {
        "manga" => println!("É manga!"),
        "Goiaba" => println!("É goiaba!"),
        "banana" => println!("banana!"),
        _ => println!("Desconhecida!"),
    }
}