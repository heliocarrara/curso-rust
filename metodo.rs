fn main(){
    let minha_casa = Casa{
        largura: 6,
        comprimento: 35,
    };

    println!("A casa tem {} de largura e {} de comprimento. Total {}", minha_casa.largura, minha_casa.comprimento, minha_casa.area());
}

impl Casa {
    fn area(&self) -> u32{
        return self.largura * self.comprimento;
    }
}

struct Casa {
    largura :u32,
    comprimento :u32
}

