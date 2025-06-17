use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Transforma uma sequência de números em um bitmask, onde cada bit representa se um número está presente.
#[inline]
pub fn seq_para_mask(seq: &[u8]) -> u32 {
    let mut mask = 0;
    for &n in seq {
        mask |= 1 << (n - 1);
    }
    mask
}

/// Operacao inversao de bitmask para sequencia
#[inline]
pub fn mask_para_seq(mask: u32) -> Vec<u8> {
    (0..25)
        .filter_map(|i| {
            if (mask & (1 << i)) != 0 {
                Some((i + 1) as u8)
            } else {
                None
            }
        })
        .collect()
}

pub fn carregar_combinacoes(path: &str, hash_set_size: usize) -> HashSet<u32> {
    let file = File::open(path).expect(&format!("Falha ao abrir: {}", path));
    let reader = BufReader::new(file);
    let mut set = HashSet::with_capacity(hash_set_size);
    for line in reader.lines() {
        let l = line.expect("Erro lendo linha");
        let nums = l
            .split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        set.insert(seq_para_mask(&nums));
    }
    set
}

pub fn get_bar(size: u64) -> ProgressBar {
    let barra = ProgressBar::new(size);
    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {msg} [{wide_bar:.cyan/blue}] {pos}/{len}")
        .expect("Template inválido");
    barra.set_style(style);
    barra
}

pub fn limpar_output() -> io::Result<()> {
    let dir = Path::new("output");
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            if name == ".gitkeep" {
                continue;
            }
        }
        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }
    Ok(())
}

/// Salva uma lista de máscaras (u32) como linhas CSV de sequências no arquivo especificado.
pub fn salvar_solucao_csv<P: AsRef<Path>>(path: P, solution: &[u32]) -> io::Result<()> {
    use std::fs::File;
    use std::io::BufWriter;
    use std::io::Write;
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for &mask in solution {
        let seq = mask_para_seq(mask);
        let line = seq
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        writeln!(writer, "{}", line)?;
    }
    Ok(())
}

/// Obtém uma seed a partir de um parâmetro opcional, variável de ambiente ou gera uma aleatória.
/// Exibe mensagens apropriadas conforme a origem da seed.
pub fn obter_seed(seed_param: Option<u64>, env_var: &str, ex_label: &str) -> u64 {
    if let Some(seed) = seed_param {
        return seed;
    }
    if let Ok(env_seed) = std::env::var(env_var) {
        if let Ok(parsed) = env_seed.parse::<u64>() {
            println!("Usando seed específica do ENV para {}: {}", ex_label, parsed);
            return parsed;
        }
    }
    let random_seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    println!("Seed gerada para {}: {}", ex_label, random_seed);
    random_seed
}
