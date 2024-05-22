fn ownership(){
    let mut palavra = String::from("Rafael");
    rouba(&mut palavra);
    println!("{}", palavra);
}

fn rouba(string: &mut String){
    string.push_str(" Lopes");
    println!("{}", string);
}

fn pattern_matching(){
    
    for i in 1..21{
        println!("{}: {}", i, match i {
            1 => "Muito pouco",
            2 | 3 => "Um pouquinho",
            4..=10 => "Um bocado",
            _ if i % 2 == 0 => "Uma boa quantidade",
            _ => "Muito"
        });
    }
}

fn erros(){
    match resultado(){
        Ok(s) => println!("String de sucesso = {}", s),
        Err(numero) => panic!("Código de erro = {}", numero)
    }
}

fn resultado() -> Result<String, u16>{
    Err(404)
}

fn main(){
    erros();
}