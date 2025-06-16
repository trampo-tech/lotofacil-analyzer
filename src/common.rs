use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::Duration;

use itertools::Itertools;

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

/// Remove combinações S15 redundantes da solução
///
/// Esta função identifica e remove combinações S15 que são completamente
/// cobertas por outras combinações já presentes na solução.
///
/// # Argumentos
///  `solution` - Vetor mutável contendo as máscaras S15 da solução
///  `original_uncovered` - Conjunto original de combinações S14 não cobertas
pub fn remover_redundantes(solution: &mut Vec<u32>, original_uncovered: &HashSet<u32>) {
    let tamanho_inicial = solution.len();

    // Create spinner for redundant removal
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"])
            .template("{spinner:.blue} {msg} [{elapsed_precise}]")
            .unwrap(),
    );
    spinner.set_message("Removendo combinações redundantes...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    let mut i = 0;

    while i < solution.len() {
        let current_mask = solution[i];
        let current_combo = mask_para_seq(current_mask);

        // Get all S14 combinations this S15 covers
        let mut current_coverage = HashSet::new();
        for &n in &current_combo {
            let sub = current_mask & !(1 << (n - 1));
            if original_uncovered.contains(&sub) {
                current_coverage.insert(sub);
            }
        }

        let covered_by_others: HashSet<u32> = solution
            .par_iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .flat_map(|(_, &other_mask)| {
                let other_combo = mask_para_seq(other_mask);
                other_combo
                    .into_iter()
                    .filter_map(|n| {
                        let sub = other_mask & !(1 << (n - 1));
                        if current_coverage.contains(&sub) {
                            Some(sub)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        // If all combinations covered by current are also covered by others, remove
        if current_coverage.is_subset(&covered_by_others) {
            let current_combo = mask_para_seq(current_mask);
            spinner.set_message(format!(
                "Removendo S15 redundante: {:?} ({}/{})",
                current_combo,
                i + 1,
                solution.len()
            ));
            solution.remove(i);
        } else {
            i += 1;
        }

        // Update progress occasionally
        if i % 1000 == 0 {
            spinner.set_message(format!(
                "Verificando redundantes... {}/{}",
                i,
                solution.len()
            ));
        }
    }

    spinner.finish_with_message(format!(
        "✓ Remoção concluída: {} -> {} S15 ({} removidas)",
        tamanho_inicial,
        solution.len(),
        tamanho_inicial - solution.len()
    ));
}

/// Otimiza a solução tentando substituir cada S15 por um melhor
///
/// Para cada combinação S15 na solução atual, esta função testa se existe
/// uma combinação alternativa que cubra mais combinações S14. Se encontrar
/// uma substituição que melhore a cobertura total, realiza a troca.
///
pub fn otimizar_por_substituicao(solution: &mut Vec<u32>, original_uncovered: &HashSet<u32>) {
    let mut melhorias = 0;

    // Create spinner for substitution optimization
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"])
            .template("{spinner:.green} {msg} [{elapsed_precise}]")
            .unwrap(),
    );
    spinner.set_message("Preparando otimização por substituição...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    // Generate all possible S15 combinations once
    spinner.set_message("Gerando todas as combinações S15 possíveis...");
    let all_s15_combos: Vec<Vec<u8>> = (1u8..=25).combinations(15).collect();

    for i in 0..solution.len() {
        let current_mask = solution[i];
        let current_combo = mask_para_seq(current_mask);

        // Update spinner message with progress
        spinner.set_message(format!(
            "Otimizando S15 {}/{} - {:?} (melhorias: {})",
            i + 1,
            solution.len(),
            current_combo,
            melhorias
        ));

        // Calculate what would be uncovered without current S15
        let mut uncovered_without_current = original_uncovered.clone();
        for (j, &other_mask) in solution.iter().enumerate() {
            if i == j {
                continue;
            }
            let other_combo = mask_para_seq(other_mask);
            for &n in &other_combo {
                let sub = other_mask & !(1 << (n - 1));
                uncovered_without_current.remove(&sub);
            }
        }

        let current_coverage = uncovered_without_current.len();

        // Parallel search for best replacement
        let best_replacement = all_s15_combos
            .par_iter()
            .filter(|combo| {
                let mask15 = seq_para_mask(combo);
                !solution.contains(&mask15)
            })
            .map(|combo| {
                let mask15 = seq_para_mask(combo);
                let mut test_uncovered = uncovered_without_current.clone();

                for &n in combo {
                    let sub = mask15 & !(1 << (n - 1));
                    test_uncovered.remove(&sub);
                }

                (test_uncovered.len(), mask15, combo.clone())
            })
            .min_by_key(|(coverage, _, _)| *coverage);

        if let Some((best_coverage, new_mask, new_combo)) = best_replacement {
            if best_coverage < current_coverage {
                spinner.set_message(format!(
                    "✓ Substituindo S15 #{}: {:?} -> {:?} (melhoria: {} combinações)",
                    i + 1,
                    current_combo,
                    new_combo,
                    current_coverage - best_coverage
                ));
                solution[i] = new_mask;
                melhorias += 1;
            }
        }

        // Progress reporting
        if (i + 1) % (solution.len() / 100).max(1) == 0 {
            spinner.set_message(format!(
                "Progresso substituição: {}/{} ({:.1}%) - {} melhorias",
                i + 1,
                solution.len(),
                (i + 1) as f64 / solution.len() as f64 * 100.0,
                melhorias
            ));
        }
    }

    spinner.finish_with_message(format!(
        "✓ Otimização por substituição concluída. {} substituições realizadas.",
        melhorias
    ));
}

/// Realiza busca local tentando substituir pares de S15 por combinações únicas
///
/// Esta função implementa uma heurística de busca local que tenta encontrar
/// uma única combinação S15 que possa substituir dois S15s existentes,
/// mantendo ou melhorando a cobertura total. Isso pode reduzir significativamente
/// o tamanho da solução final.
///
pub fn otimizar_local_search(solution: &mut Vec<u32>, original_uncovered: &HashSet<u32>) {
    let mut melhorias = 0;
    let mut restart = true;
    let mut iteration = 0;

    // Create spinner for local search optimization
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"])
            .template("{spinner:.yellow} {msg} [{elapsed_precise}]")
            .unwrap(),
    );
    spinner.set_message("Iniciando busca local (2->1)...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    // Pre-generate combinations for parallel processing
    spinner.set_message("Gerando combinações para busca local...");
    let all_s15_combos: Vec<Vec<u8>> = (1u8..=25).combinations(15).collect();

    while restart {
        restart = false;
        iteration += 1;
        spinner.set_message(format!(
            "Busca local - Iteração {} ({} S15s, {} melhorias)",
            iteration,
            solution.len(),
            melhorias
        ));

        'outer: for i in 0..solution.len() {
            for j in (i + 1)..solution.len() {
                let mask1 = solution[i];
                let mask2 = solution[j];
                let combo1 = mask_para_seq(mask1);
                let combo2 = mask_para_seq(mask2);

                // Calculate coverage without these two S15s
                let mut uncovered_without_pair = original_uncovered.clone();
                for (k, &other_mask) in solution.iter().enumerate() {
                    if k == i || k == j {
                        continue;
                    }
                    let other_combo = mask_para_seq(other_mask);
                    for &n in &other_combo {
                        let sub = other_mask & !(1 << (n - 1));
                        uncovered_without_pair.remove(&sub);
                    }
                }

                let pair_coverage = uncovered_without_pair.len();

                // Parallel search for single S15 replacement
                let replacement = all_s15_combos
                    .par_iter()
                    .filter(|combo| {
                        let mask15 = seq_para_mask(combo);
                        !solution.contains(&mask15)
                    })
                    .find_any(|combo| {
                        let mask15 = seq_para_mask(combo);
                        let mut test_uncovered = uncovered_without_pair.clone();

                        for &n in *combo {
                            let sub = mask15 & !(1 << (n - 1));
                            test_uncovered.remove(&sub);
                        }

                        test_uncovered.len() <= pair_coverage
                    });

                if let Some(combo) = replacement {
                    let mask15 = seq_para_mask(combo);
                    let mut test_uncovered = uncovered_without_pair.clone();
                    for &n in combo {
                        let sub = mask15 & !(1 << (n - 1));
                        test_uncovered.remove(&sub);
                    }

                    spinner.set_message(format!(
                        "✓ Substituindo par S15 #{},{}: {:?},{:?} -> {:?} (melhoria: {})",
                        i + 1,
                        j + 1,
                        combo1,
                        combo2,
                        combo,
                        pair_coverage - test_uncovered.len()
                    ));
                    solution[j] = mask15;
                    solution.remove(i);
                    melhorias += 1;
                    restart = true;
                    break 'outer;
                }
            }
        }
    }

    spinner.finish_with_message(format!(
        "✓ Busca local concluída. {} melhorias realizadas.",
        melhorias
    ));
}

/// Executa todas as otimizações disponíveis na solução S15
///
/// Esta função coordena a execução de todas as técnicas de otimização
/// disponíveis para reduzir o tamanho da solução final. As otimizações
/// são aplicadas em sequência para maximizar a redução.
///
pub fn otimizar_solucao_completa(
    solution: &mut Vec<u32>,
    original_uncovered: &HashSet<u32>,
) -> (usize, usize, std::time::Duration) {
    use std::time::Instant;

    println!("\n=== INICIANDO FASE DE OTIMIZAÇÃO ===");
    let opt_start = Instant::now();
    let initial_size = solution.len();

    // Etapa 1: Remove combinações redundantes
    remover_redundantes(solution, original_uncovered);
    println!("Após remoção de redundantes: {} S15", solution.len());

    // Etapa 2: Tenta substituições melhores
    otimizar_por_substituicao(solution, original_uncovered);
    println!("Após substituições: {} S15", solution.len());

    // Etapa 3: Busca local (2->1)
    otimizar_local_search(solution, original_uncovered);
    println!("Após busca local: {} S15", solution.len());

    let opt_elapsed = opt_start.elapsed();
    let final_size = solution.len();

    println!(
        "=== OTIMIZAÇÃO CONCLUÍDA ===\nTempo: {:.2?}\nRedução: {} -> {} S15 ({:.1}% redução)",
        opt_elapsed,
        initial_size,
        final_size,
        if initial_size > 0 {
            (1.0 - final_size as f64 / initial_size as f64) * 100.0
        } else {
            0.0
        }
    );

    (initial_size, final_size, opt_elapsed)
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
