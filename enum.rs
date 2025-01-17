enum Animal {
    Cachoro,
    Gato,
    Calango,
}

fn main(){
    let meu_pet: Animal = Animal::Cachoro;

    match meu_pet {
        Animal::Cachoro => println!("O seu é Cachoro!"),
        Animal::Gato => println!("O seu é um gato!"),
        Animal::Calango => println!("O seu é o calango!"),
        _ => println!("Não achei esse!"),
    }
}