fn main(){
    
    let variavel:i32 = 128;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("variavel = {}, tamanho = {} bytes", decimal, std::mem::size_of_val(&variavel));

    let mut booleana:bool = false;
    booleana = true;  
    println!("Booleana = {} Tamanho booleana = {}",booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'A';
    println!("Letra = '{}' Tamanho char = {}",letra, std::mem::size_of_val(&letra));
}