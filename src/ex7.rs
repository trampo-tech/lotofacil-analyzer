use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn contar_linhas(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}

pub fn executar() -> io::Result<()> {
    let custo_unitario = 3.0_f64;

    let jogos = [
        ("SB15_14", "output/SB15_14.csv"),
        ("SB15_13", "output/SB15_13.csv"),
        ("SB15_12", "output/SB15_12.csv"),
        ("SB15_11", "output/SB15_11.csv"),
    ];

    println!("Cálculo de custo financeiro (R$ 3,00 por jogo):\n");

    for &(nome, path) in &jogos {
        let n = contar_linhas(path)?;
        let custo = (n as f64) * custo_unitario;
        println!(
            "{:<8} → {:>10} jogos → Custo: R$ {:.2}",
            nome,
            n,
            custo
        );
    }

    println!("\nCálculo concluído.");
    Ok(())
}
