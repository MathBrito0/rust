use std::io;

fn main() {
    println!("Digite quantos números quiser: ");

    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler a quantidade.");

    let quantidade: usize = entrada
        .trim()
        .parse()
        .expect("Digite um número válido.");

    let mut numeros: Vec<u32> = Vec::new();

    for _ in 0..quantidade {
        let mut valor = String::new();

        println!("Digite um número: ");
        io::stdin()
            .read_line(&mut valor)
            .expect("Erro ao ler o número.");

        let valor: u32 = valor
            .trim()
            .parse()
            .expect("Digite um valor válido.");

        numeros.push(valor);
    }

    let maior = numeros
        .iter()
        .copied()
        .max()
        .expect("Nenhum número foi digitado.");

    println!("O maior número é: {maior}");
}