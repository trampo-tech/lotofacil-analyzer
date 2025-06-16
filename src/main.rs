use std::io;

mod common;
mod ex1;
mod ex2;
mod ex3;
mod ex4;
mod ex5;
mod ex6;
mod ex7;

fn main() {
    println!("Escolha o exercício (1 a 7) ou 'c' para limpar a pasta 'output':");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => ex1::executar(),
        "2" => ex2::executar(),
        "c" | "C" => {
            common::limpar_output().expect("Falha ao limpar pasta 'output'");
            println!("Pasta 'output' limpa!");
        }
        _ => println!("Opção inválida"),
    }
}
