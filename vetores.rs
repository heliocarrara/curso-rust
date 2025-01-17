fn main (){
    let v = vec![23, 24, 25];

    println!("O valor do indice {} é {}", 2, v[2]);


    let mut vetor: Vec<u32> = Vec::new();

    vetor.push(89);
    vetor.push(49);
    vetor.push(66);

    println!("Após adicionar: O valor do indice {} é {}", 1, vetor[1]);

    vetor.remove(1);

    println!("Após excluir: O valor do indice {} é {}", 1, vetor[1]);
}