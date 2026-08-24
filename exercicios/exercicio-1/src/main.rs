/*
 * Exercício 1 - Contagem de vogais e consoantes.
 *
 * Este é um programa que verifica quantas vogais e quantas consoantes tem em um input.
 *
 * Autor: [Matheus]
 * Data: [24/08/2026]
 */

use std::io;

fn main() {
    println!("Digite: ");
    let mut buffer: String = String::new();
    let mut vogal: u32 = 0;
    let mut consoante: u32 = 0;
    io::stdin()
        .read_line(&mut buffer).expect("Erro ao ler comando!");
    for i in buffer.trim().chars() {
        if i == 'a' 
        || i == 'A' 
        || i == 'e' 
        || i == 'E' 
        || i == 'i' 
        || i == 'I' 
        || i == 'o'
        || i == 'O' 
        || i == 'u' 
        || i == 'U' {
            vogal += 1;
        } else {
            consoante += 1;
        }
    }
    println!("Vogais: {vogal}, Consoantes {consoante}");
}


/*Polimentos possíveis:

else if i.is_alphabetic() {}

if "aeiouAEIOU".contains(i) {}

let mut buffer = String::new();

io::stdin()
    .read_line(&mut buffer)
    .expect("Erro ao ler comando!")
    
let mut vogais: u32 = 0;
let mut consoantes: u32 = 0*/