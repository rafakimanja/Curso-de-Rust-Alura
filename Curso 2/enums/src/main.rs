fn main() {
    cores();
}

fn fim_de_semana(dia_semana: DiaDaSemana) -> bool
{
    match dia_semana{
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false
    }
}

enum DiaDaSemana{
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

#[allow(dead_code)]
enum Color{
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor{cyan: u8, magenta: u8, yellow: u8, black: u8}
}

fn cores(){

    let cor = Color::CymkColor{cyan: 100, magenta: 50, yellow: 70, black: 255};

    println!(
        "Cor = {}",
        match cor{
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "azul",
        Color::RgbColor(0, 0, 0 ) | Color::CymkColor{cyan:_, magenta:_, yellow:_, black: 255} => "preto",
        Color::RgbColor(_, _, _)  => "RGB desconhecido",
        Color::CymkColor{cyan:_, magenta:_, yellow:_, black:_} => "Cymk desconhecido"
    });
}