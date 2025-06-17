use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn contar_linhas(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}

pub fn executar() -> io::Result<()> {
    let custo_unitario = 3.0_f64;
    let combinacoes_dir = "output/combinacoes";
    println!("\x1b[2J\x1b[H"); // Limpa tela (ANSI escape)
    println!("📊 Cálculo de Custo por Análise Lotofácil\n");

    let entries = match fs::read_dir(combinacoes_dir) {
        Ok(e) => e,
        Err(_) => {
            println!("[AVISO] Diretório de combinações não encontrado.");
            return Ok(());
        }
    };
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
        let ex_label = if let Some(ex) = nome.split('_').nth(0) {
            match ex {
                "SB15" => {
                    // Tenta inferir o número do ex a partir do nome do arquivo
                    if let Some(suf) = nome.split('_').nth(1) {
                        match suf {
                            "11" => "ex5",
                            "12" => "ex4",
                            "13" => "ex3",
                            "14" => "ex2",
                            _ => "ex?",
                        }
                    } else {
                        "ex?"
                    }
                }
                _ => "ex?",
            }
        } else {
            "ex?"
        };
        let n = contar_linhas(path.to_str().unwrap())?;
        let custo = (n as f64) * custo_unitario;
        println!(
            "{:<30} | {:<4} | {:>10} jogos | Custo: R$ {:>8.2}",
            nome, ex_label, n, custo
        );
    }

    println!("\n---------------------------------");
    println!("Cálculo concluído.");
    Ok(())
}
