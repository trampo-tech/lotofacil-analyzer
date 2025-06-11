use std::io;

mod common;
mod exercicio1;
mod exercicio2;
mod exercicio3;
mod exercicio4;
mod exercicio5;
mod exercicio6;
mod exercicio7;

fn main() {
    println!("Escolha o exercício (1 a 7):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => exercicio1::executar(),
        _ => println!("Exercício inválido"),
    }
}