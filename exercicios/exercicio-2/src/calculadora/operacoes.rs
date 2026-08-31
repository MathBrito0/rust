pub fn soma(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtracao(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiplicacao(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divisao(a: i32, b: i32) -> i32 {
    if b == 0 {
        println!("Erro: divisão por zero!");
        return 0;
    }
    a / b
}