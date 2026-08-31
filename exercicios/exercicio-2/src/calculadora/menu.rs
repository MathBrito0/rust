use std::io;
use super::operacoes;

pub fn exibir_menu() {
    loop {
        println!("\n=== Calculadora ===");
        println!("1 - Soma");
        println!("2 - Substração");
        println!("3 - Multiplicação");
        println!("4 - Divisão");
        println!("5 - Sair");
        println!("Escolha uma opção: ");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro ao ler a opção");

        let opcao: u32 = match opcao.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Opção inválida!");
                continue;
            }
        };
        
        if opcao == 5 {
            println!("Saindo...");
            break;
        }

        if opcao < 1 || opcao > 4 {
            println!("Opção inválida!");
            continue;
        }

        let a = ler_numero("Digite o primeiro número: ");
        let b = ler_numero("Digite o segundo número: ");

        let resultado = match opcao {
            1 => operacoes::soma(a, b),
            2 => operacoes::subtracao(a, b),
            3 => operacoes::multiplicacao(a, b),
            4 => operacoes::divisao(a, b),
            _ => unreachable!(),
        };

        println!("Resultado: {}", resultado);
    }    

fn ler_numero(mensagem: &str) -> i32 {
    println!("{}", mensagem);
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler o número!");
    entrada.trim().parse().expect("Digite um número inteiro válido.")
    }
}