fn main (){
    // operadores relacionais

    let a :u32 = 10;
    let b :u32 = 15;

    if a > b {
        println!("{} é maior que {}", a, b);
    } else {
        println!("{} é menor que {}", a, b);
    }
    
    if a != b {
        println!("{} é diferente de {}", a, b);
    } else {
        println!("{} é igual a {}", a, b);
    }

    if a >= b {
        println!("{} é maior ou igual a {}", a, b);
    } else {
        println!("{} é menor que {}", a, b);
    }

    if a <= b {
        println!("{} é menor ou igual a {}", a, b);
    } else {
        println!("{} é maior que {}", a, b);
    }
}