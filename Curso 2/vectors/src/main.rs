fn main() {
    vectors();
}


fn vectors(){
    
    let mut nomes:Vec<&str> = Vec::with_capacity(4);

    nomes.push("Rafael");
    nomes.push("Lopes");
    nomes.push("Ens");
    nomes.push("Software Development"); //adicionar um valor

    println!("Sobrenome = {}", nomes[1]);

    for nome in &nomes
    {
        println!("{}", nome);
    }

    println!("nome em 10 = {}", match &nomes.get(10){
        Some(n) => *n,
        None => "Não existe"
    });

    nomes.pop(); //remove o ultimo valor do vetor

    while let Some(nome) = nomes.pop()
    {
        println!("nome removido = {}", nome);
    }
}