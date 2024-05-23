fn main() {
    conta_corrente();
}

struct Titular{
    nome: String,
    sobrenome: String
}

struct Conta {
    titular: Titular, 
    saldo: f64
}

impl Conta {
    fn sacar(&mut self, valor: f64){
        
        self.saldo -= valor;
    }
}

fn conta_corrente(){

    let mut conta: Conta = Conta{titular: Titular{nome: String::from("Rafael"), sobrenome: String::from("Lopes Ens")}, saldo: 3.75};

    println!("Dados da conta: Titular = {} {}, Saldo = {}", conta.titular.nome, conta.titular.sobrenome, conta.saldo);

    conta.sacar(2.50);

    println!("Dados da conta: Titular = {} {}, Saldo = {}", conta.titular.nome, conta.titular.sobrenome, conta.saldo);
}
