fn main(){
    println!("Testanto o for...");

    for x in 10..15{
        println!("Esta é a linha {}", x);
    }

    println!("---------------");
    println!("Testanto o while...");
    let mut y = 0;

    while y < 5 {
        println!("Esta é a linha {}", y);
        y += 1;
    }

    println!("---------------");
    println!("Testanto o loop...");
    let mut z = 0;

    loop {
        println!("Esta é a linha {}", z);
        z += 1;
        if z == 5 {
            break;
        }
    }
}