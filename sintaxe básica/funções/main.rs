fn soma(a:i32, b:i32) -> i32
{
    println!("{} + {} = {}", a, b, a+b);
    a+b
}

fn main(){
    println!("soma = {}", soma(2, 2));
}