fn main(){
    let salario :u32 = 1200;

    if salario > 1000 {
        println!("Você ganha mais de 1000 reais");
    } else {
        println!("Você ganha menos de 1000 reais");
    }

    if salario < 1400{
        println!("Você ganha menos de 1400 reais");
    } else if salario >= 1400 && salario > 2000 {
        println!("Você ganha 1400 reais e menos que 2000 reais");
    }


    let idade :u32 = 18;

    let sou_maior_de_idade = if idade >= 18 {
        true
    } else {
        false
    };

    println!("Sou maior de idade? {}", sou_maior_de_idade);
    let sou_menor = if idade < 18 {"Sim"} else {"Não"};

    println!("Sou menor de idade? {}", sou_menor);

    let fruta :&str = "Maçã";
    
    let estado_fruta :&str = "Madura";

        if {fruta == "Maçã" || fruta == "Banana"} && {estado_fruta == "Madura"} {
            println!("A fruta é boa para vitaminas");
        } else {
            println!("A fruta não é boa para vitaminas, mas é boa para outras coisas");
        }
    
    if !{salario == 12000}{
        println!("Você não ganha 12000 reais");
    }
    else {
        println!("Você ganha 12000 reais");
    }
}