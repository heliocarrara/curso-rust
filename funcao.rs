fn quadrado(x: u32) -> u32{
    return x * x;
}

fn main(){
    let num :u32 = 21;

    println!("O quadrado do numero {} é {}", num, quadrado(num))
}