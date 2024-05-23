const PI:f32 = 3.14;
static mut VARIAVEL_GLOBAL:u8 = 1;

fn main(){
    println!("PI = {}", PI);

    unsafe{
        println!("Variavel Global = {}", VARIAVEL_GLOBAL);
    }

}