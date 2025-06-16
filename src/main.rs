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
    println!("Escolha o exercício (1 a 7), 'c' para limpar a pasta 'output', ou 'q' para sair:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => ex1::executar(),
        "2" => ex2::executar(),
        "3" => ex3::executar(),
        "4" => ex4::executar(),
        "5" => ex5::executar(),
        "6" => ex6::executar(),
        "7" => ex7::executar().expect("Falha ao calcular custo financeiro"),
        "c" | "C" => {
            common::limpar_output().expect("Falha ao limpar pasta 'output'");
            println!("Pasta 'output' limpa!");
        }
        "q" | "Q" => {
            println!("Saindo...");
        }
        _ => println!("Opção inválida"),
    }
}
