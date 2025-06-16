use itertools::Itertools;
use rayon::prelude::*;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::time::Instant;

fn gerar_e_salvar(k: usize) {
    let inicio = Instant::now();
    let nome_arquivo = format!("output/saida_S{}.csv", k);

    let file = File::create(&nome_arquivo).expect("Erro ao criar o arquivo");
    let mut writer = BufWriter::new(file);

    for combinacao in (1u8..=25).combinations(k) {
        let linha = combinacao
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(",");
        writeln!(writer, "{}", linha).expect("Erro ao escrever no arquivo");
    }

    println!(
        "S{} salvo em '{}'. Tempo: {:.2?}",
        k,
        nome_arquivo,
        inicio.elapsed()
    );
}

pub fn executar() {
    create_dir_all("output").expect("Erro ao criar pasta 'output'");

    let tamanhos = [15, 14, 13, 12, 11];
    let inicio_total = Instant::now();

    println!("Iniciando geração paralela das combinações...\n");

    tamanhos.par_iter().for_each(|&k| {
        gerar_e_salvar(k);
    });

    println!(
        "\nGeração completa de todas as combinações! Tempo total: {:.2?}",
        inicio_total.elapsed()
    );
}
