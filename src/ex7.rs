use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

fn contar_linhas(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}

pub fn executar() -> io::Result<()> {
    let custo_unitario = 3.0_f64;
    let combinacoes_dir = "output/combinacoes";
    println!("Cálculo de custo financeiro (R$ 3,00 por jogo):\n");

    let entries = fs::read_dir(combinacoes_dir)?;
    let mut arquivos: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .collect();
    arquivos.sort_by_key(|e| e.file_name());

    for entry in arquivos {
        let path = entry.path();
        let nome = path.file_name().unwrap().to_string_lossy();
        if nome == ".gitkeep" {
            continue;
        }
        let n = contar_linhas(path.to_str().unwrap())?;
        let custo = (n as f64) * custo_unitario;
        println!("{:<30} → {:>10} jogos → Custo: R$ {:.2}", nome, n, custo);
    }

    println!("\nCálculo concluído.");
    Ok(())
}
