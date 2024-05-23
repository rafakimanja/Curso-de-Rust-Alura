fn main() {
    
    let notas:[f32; 4]= [6.5; 4];
    let indice: usize = 2;

    // for nota in notas {
    //     println!("nota = {}", nota);
    // }

    for indice in 0..notas.len(){
        println!("{}° nota = {}",indice+1, notas[indice]);
    }

    println!("Posição {} = {}", indice, notas[indice]);

    matriz();
}


fn matriz(){

    let matriz = [
        [1.1, 1.2, 1.3],
        [0.22, 0.25, 0.27]
    ];

    for linha in matriz{
        for item in linha{
            print!("{}, ", item);
        }
        println!("");
    }
}
