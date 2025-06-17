use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

fn count_lines(file_path: &str) -> Result<usize, String> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("Arquivo não encontrado: {}", file_path));
    }
    let file = File::open(path).map_err(|e| format!("Erro ao abrir {}: {}", file_path, e))?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}

pub fn executar() {
    cliclack::clear_screen().unwrap_or_default();
    cliclack::outro("Resultados da Análise Lotofácil").unwrap_or_default();

    let combinacoes_dir = "output/combinacoes";
    let entries = match fs::read_dir(combinacoes_dir) {
        Ok(e) => e,
        Err(_) => {
            cliclack::log::warning("Diretório de combinações não encontrado.").unwrap_or_default();
            return;
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
        match count_lines(path.to_str().unwrap()) {
            Ok(line_count) => {
                cliclack::log::info(format!("{}: {} combinações S15", nome, line_count))
                    .unwrap_or_default();
            }
            Err(e) => {
                cliclack::log::warning(format!("{}: {}", nome, e)).unwrap_or_default();
            }
        }
    }
    cliclack::outro("---------------------------------").unwrap_or_default();
}
