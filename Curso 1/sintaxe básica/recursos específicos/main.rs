fn nova_funcao(){
    let variavel:i32 = 128;
    println!("variavel = {}", variavel);

    let variavel:i32 = 130;
    println!("variavel = {}", variavel);
}

fn sombra(){
    let a = 123;

    {
        let a = 888;
        let b = 456;
        println!("dentro 'b' = {}", b);
        println!("variavel dentro 'a' = {}", a);
    }

    //println!("fora 'b' = {}", b); //não vai funcionar

    println!("variavel 'a' = {}", a);
}

fn main(){
    nova_funcao();
    sombra();
}