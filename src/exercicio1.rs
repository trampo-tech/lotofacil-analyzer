use itertools::Itertools;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::time::Instant;

fn gerar_combinacoes(k: usize) -> Vec<Vec<u8>> {
    (1u8..=25)
        .combinations(k)
        .collect()
}

fn salvar_combinacoes(nome_arquivo: &str, combinacoes: &[Vec<u8>]) {
    let file = File::create(nome_arquivo).expect("Erro ao criar o arquivo");
    let mut writer = BufWriter::new(file);

    for linha in combinacoes {
        let linha_str = linha
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");

        writeln!(writer, "{}", linha_str).expect("Erro ao escrever no arquivo");
    }
}

pub fn executar() {
    create_dir_all("output").expect("Erro ao criar pasta 'output'");

    let tamanhos = [15, 14, 13, 12, 11];
    let inicio_total = Instant::now();

    for &k in &tamanhos {
        println!("Gerando combinações de {} números...", k);
        let inicio = Instant::now();

        let combinacoes = gerar_combinacoes(k);

        println!(
            "Total de combinações para {} números: {}",
            k,
            combinacoes.len()
        );

        let nome_arquivo = format!("output/saida_S{}.csv", k);
        salvar_combinacoes(&nome_arquivo, &combinacoes);

        println!(
            "Combinações de {} salvas em '{}'. Tempo: {:.2?}\n",
            k,
            nome_arquivo,
            inicio.elapsed()
        );
    }

    println!(
        "Geração completa! Tempo total: {:.2?}",
        inicio_total.elapsed()
    );
}
