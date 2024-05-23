fn main(){

    let idade:u8 = 18;
    let responsavel_autorizou = true;
    let eh_maior = idade >= 18;

    if eh_maior {
        println!("Pode entrar na balada");
    }
    else if idade >= 16 && responsavel_autorizou{
        println!("Pode entrar na balada");
    }
    else{
        println!("não pode entrar na bala");
    }

    let condicao = if eh_maior { "maior" } else { "menor" };

    println!("Condição = {}", condicao);
}