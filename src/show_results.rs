use std::fs::File;
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
    cliclack::outro("📊 Resultados da Análise Lotofácil").unwrap_or_default();

    let files_and_descs = vec![
        ("output/SB15_14.csv", "Cobertura S14 (Ex2)"),
        ("output/SB15_13.csv", "Cobertura S13 (Ex3)"),
        ("output/SB15_12.csv", "Cobertura S12 (Ex4)"),
        ("output/SB15_11.csv", "Cobertura S11 (Ex5)"),
    ];

    for (file_path, description) in files_and_descs {
        match count_lines(file_path) {
            Ok(line_count) => {
                cliclack::log::info(format!(
                    "{}: {} combinações S15",
                    description, line_count
                ))
                .unwrap_or_default();
            }
            Err(e) => {
                cliclack::log::warning(format!("{}: {}", description, e)).unwrap_or_default();
            }
        }
    }
    cliclack::outro("---------------------------------").unwrap_or_default();
}
