fn tabuada(multiplicador:u8){
    for i in 1..11{
        println!("{} x {} = {}", i, multiplicador, i*multiplicador);
    }
}

fn main(){
    tabuada(6);
}