use std::io;
use super::operacoes;

pub fn exibir_menu() {
    loop {
        let mut buffer = String::new();
        println!("Digite o primeiro número (ou 'sair' para encerrar): ");
        io::stdin().read_line(&mut buffer).unwrap();
        let buffer = buffer.trim();

        if buffer == "sair" {
            println!("Encerrando...");
            break;
        }

        let valor1: i32 = buffer.parse().expect("Erro!");
        println!("O primeiro número foi = {valor1}");

        let mut buffer2 = String::new();
        println!("Digite o segundo número: ");
        io::stdin().read_line(&mut buffer2).unwrap();
        let valor2: i32 = buffer2.trim().parse().expect("Erro!");
        println!("O segundo número foi = {valor2}");

        let mut buffer3 = String::new();
        println!("Digite a operação desejada (+, -, x, /): ");
        io::stdin().read_line(&mut buffer3).unwrap();
        let buffer3 = buffer3.trim();

        if buffer3 == "+" {
            let resultado = operacoes::soma(valor1, valor2);
            println!("O valor da soma é --> {resultado}");
        } else if buffer3 == "-" {
            let resultado = operacoes::subtracao(valor1, valor2);
            println!("O valor da subtração é --> {resultado}");
        } else if buffer3 == "x" {
            let resultado = operacoes::multiplicacao(valor1, valor2);
            println!("O valor do produto é --> {resultado}");
        } else if buffer3 == "/" {
            let resultado = operacoes::divisao(valor1, valor2);
            println!("O valor da divisão é --> {resultado}");
        } else {
            println!("Erro no cálculo!");
        }
    }
}