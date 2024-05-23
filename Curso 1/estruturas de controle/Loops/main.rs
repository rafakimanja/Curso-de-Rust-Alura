fn tabuada(multiplicador:u8){
    
    let mut contador:u8 = 0;

    while contador < 10{

        contador += 1;

        if contador == 5{
            continue;
        }

        println!("{} x {} = {}", contador, multiplicador, contador*multiplicador);
    }

    contador = 0;

    loop {
        contador += 1;

        println!("{} x {} = {}", contador, multiplicador, contador*multiplicador);

        if contador == 10{
            break;
        }
    }
}


fn main(){
    tabuada(5);
}