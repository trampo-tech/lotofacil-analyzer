use cliclack::{outro, select};

mod common;
mod ex1;
mod ex2;
mod ex3;
mod ex4;
mod ex5;
mod ex6;
mod ex7;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Escolha o exercício (1 a 7):");

    loop {
        let exercicio = select("Escolha qual exercício executar:")
            .item(1, "Exercicio 1", "")
            .item(2, "Exercicio 2", "")
            .item(3, "Exercicio 3", "")
            .item(4, "Exercicio 4", "")
            .item(5, "Exercicio 5", "")
            .item(6, "Exercicio 6", "")
            .item(7, "Exercicio 7", "")
            .item(99, "Limpar", "")
            .item(0, "Sair", "")
            .interact()?;
        outro("")?;
        match exercicio {
            1 => ex1::executar(),
            2 => ex2::executar(),
            3 => ex3::executar(),
            4 => ex4::executar(),
            5 => ex5::executar(),
            6 => ex6::executar(),
            7 => ex7::executar().expect("Falha ao calcular custo financeiro"),
            99 => {
                common::limpar_output().expect("Falha ao limpar pasta 'output'");
                println!("Pasta 'output' limpa!");
            }
            0 => {
                println!("Saindo...");
                break Ok(());
            }
            _ => println!("Opção inválida."),
        }
        return Ok(());
    }
}
