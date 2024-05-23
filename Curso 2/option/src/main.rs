fn main() {
    conteudo_opcional();
}

fn conteudo_opcional(){
    let conteudo_arquivo = ler_arquivo(String::from("c:/users/documentos"));
    
    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("arquivo não existe!")
    };

    println!("{:?}", conteudo_arquivo);

    if let Some(valor) = conteudo_arquivo{ println!("Substituto para o match, arquivo = {}", valor) };
}

fn ler_arquivo(caminho_arquivo: String) -> Option<String>
{
    Some(String::from("estrogonogodof"))
    //None
}