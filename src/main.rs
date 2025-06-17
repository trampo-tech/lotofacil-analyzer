use cliclack::{input, outro, select};

mod common;
mod ex1;
mod ex2;
mod ex3;
mod ex4;
mod ex5;
mod ex6;
mod ex7;
mod show_results; // Adiciona o novo módulo
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    cliclack::intro("Lotofácil Analyzer")?;
    loop {
        let exercicio: u32 = select("Escolha qual exercício executar:")
            .item(1, "Exercicio 1", "")
            .item(2, "Exercicio 2", "")
            .item(3, "Exercicio 3", "")
            .item(4, "Exercicio 4", "")
            .item(5, "Exercicio 5", "")
            .item(6, "Exercicio 6", "")
            .item(7, "Exercicio 7", "")
            .item(
                8,
                "Mostrar Resultados",
                "Exibe o número de combinações S15 para os exercícios 2-5",
            ) // Nova opção
            .item(99, "Limpar", "")
            .item(0, "Sair", "")
            .interact()?;
        match exercicio {
            1 => ex1::executar(),
            2 => {
                let seed_str: String = input("Forneça uma seed (opcional):")
                    .placeholder("Pressione Enter para gerar uma seed aleatória")
                    .required(false)
                    .interact()?;
                let seed_input: Option<u64> = if seed_str.is_empty() {
                    None
                } else {
                    seed_str.parse::<u64>().ok()
                };
                outro("")?;
                ex2::executar(seed_input);
            }
            3 => {
                let seed_str: String = input("Forneça uma seed para ex3 (opcional):")
                    .placeholder("Pressione Enter para gerar uma seed aleatória")
                    .required(false)
                    .interact()?;
                let seed_input: Option<u64> = if seed_str.is_empty() {
                    None
                } else {
                    seed_str.parse::<u64>().ok()
                };
                outro("")?;
                ex3::executar(seed_input);
            }
            4 => {
                let seed_str: String = input("Forneça uma seed para ex4 (opcional):")
                    .placeholder("Pressione Enter para gerar uma seed aleatória")
                    .required(false)
                    .interact()?;
                let seed_input: Option<u64> = if seed_str.is_empty() {
                    None
                } else {
                    seed_str.parse::<u64>().ok()
                };
                outro("")?;
                ex4::executar(seed_input);
            }
            5 => {
                let seed_str: String = input("Forneça uma seed para ex5 (opcional):")
                    .placeholder("Pressione Enter para gerar uma seed aleatória")
                    .required(false)
                    .interact()?;
                let seed_input: Option<u64> = if seed_str.is_empty() {
                    None
                } else {
                    seed_str.parse::<u64>().ok()
                };
                outro("")?;
                ex5::executar(seed_input);
            }
            6 => ex6::executar(),
            7 => ex7::executar().expect("Falha ao calcular custo financeiro"),
            8 => show_results::executar(),
            99 => {
                common::limpar_output().expect("Falha ao limpar pasta 'output'");
                println!("Pasta 'output' limpa!");
            }
            0 => {
                outro("👋 Saindo...")?;
                break Ok(());
            }
            _ => outro("Opção inválida. Tente novamente.")?,
        }
    }
}
