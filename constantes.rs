fn main(){
    const CONST_1 :char = 'A';
    const CONST_2 :&str = "Hola dev";
    const CONST_3 :u32 = 100;
    const CONST_4 :f32 = 3.14;
    const CONST_5 :bool = true;
    const CONST_6 :[i32; 3] = [1, 2, 3];
    const CONST_7 :usize = 100_000_000;

    println!("CONST_1 do tipo char: {}", CONST_1);
    println!("CONST_2 do tipo &str: {}", CONST_2);
    println!("CONST_3 do tipo u32: {}", CONST_3);
    println!("CONST_4 do tipo f32: {}", CONST_4);
    println!("CONST_5 do tipo bool: {}", CONST_5);
    println!("CONST_6 do tipo [i32; 3]: {:?}", CONST_6);
    println!("CONST_7 do tipo usize: {}", CONST_7);
}