fn funcao(){

    let linguagem = "Java";
    let proposito = match linguagem{
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido"
    };

    println!("O proposito de {} e {}", linguagem, proposito);
}

fn main(){
    funcao();
}