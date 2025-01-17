fn main(){
    let a = 20;
    let b = 4;
    let c = 6;

    let soma = a + b + c;
        let subtracao = a - b - c;
            let multiplicacao = a * b * c;
                let resto = a % c;
                    let divisao = a / b;

    println!("Soma: {}", soma);
    println!("Subtração: {}", subtracao);
    println!("Multiplicação: {}", multiplicacao);
    println!("Resto: {}", resto);
    println!("Divisão: {}", divisao);

    println!("-------------------");

    println!("As debaixo são as mesmas operações, mas sem variáveis intermediárias");

    println!("-------------------");

    println!("Soma: {}", a + b + c);
    println!("Subtração: {}", a - b - c);
    println!("Multiplicação: {}", a * b * c);
    println!("Resto: {}", a % c);
    println!("Divisão: {}", a / b);
}