/*
 * Exercício 2 - Calculadora.
 *
 * Este programa solicita dois números inteiros assinados de 32 bits do usuário e calcula
 uma das 4 operações matemáticas básicas.*

 * Autor: [Matheus]
 * Data: [26/08/2026]
 */



use std::io;

 fn main() {
    let mut buffer = String::new();
    println!("Digite o primeiro número: ");
    io::stdin().read_line(&mut buffer).unwrap();
    let valor1: i32 = buffer.trim().parse().expect("Erro!");
    println!("O primeiro número foi = {valor1}");

    let mut buffer2 = String::new();
    println!("Digite o segundo número: ");
    io::stdin().read_line(&mut buffer2).unwrap();
    let valor2: i32 = buffer2.trim().parse().expect("Erro!");
    println!("O segundo número foi = {valor2}");

    let mut buffer3 = String::new();
    println!("Digite a operação desejada: ");
    io::stdin().read_line(&mut buffer3).unwrap();
    buffer3 = buffer3.trim().to_string();
    if buffer3.as_str() == "+" {
        let resultado = valor1+valor2;
        println!("O valor da soma é --> {resultado}");
    } else if buffer3.as_str() == "-" {
        let resultado = valor1-valor2;
        println!("O valor da subtração é --> {resultado}");
    } else if buffer3.as_str() == "x" {
        let resultado = valor1*valor2;
        println!("O valor do produto é --> {resultado}");
    } else if buffer3.as_str() == "/" {
        let resultado = valor1/valor2;
        println!("O valor da divisão é --> {resultado}");
    } else {
        println!("Erro no cálculo!");
    }
    
 }