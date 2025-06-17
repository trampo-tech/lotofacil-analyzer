use crate::common::{carregar_combinacoes, get_bar, mask_para_seq, seq_para_mask};
use itertools::Itertools;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use std::fs::{File, create_dir_all};
use std::time::Instant;

pub fn executar(seed_param: Option<u64>) {
    create_dir_all("output").expect("Não pôde criar output");

    let seed = seed_param.unwrap_or_else(|| {
        std::env::var("LOTOFACIL_SEED")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or_else(|| {
                let random_seed = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64;
                if seed_param.is_none() && std::env::var("LOTOFACIL_SEED").is_err() {
                    println!("Seed gerada para ex2: {}", random_seed);
                }
                random_seed
            })
    });

    if seed_param.is_some() {
    } else if std::env::var("LOTOFACIL_SEED").is_ok() {
        println!("Usando seed específica do ENV para ex2: {}", seed);
    }

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    println!("Carregando S14...");
    let original_uncovered_s14 = carregar_combinacoes("output/saida_S14.csv", 4_500_000);
    let total_s14_to_cover = original_uncovered_s14.len();
    println!("S14 carregado: {} combinações a cobrir", total_s14_to_cover);

    if total_s14_to_cover == 0 {
        println!("Nenhuma combinação S14 para cobrir. Saindo.");
        let out_path_seeded = format!("output/SB15_14_seed_{}.csv", seed);
        File::create(&out_path_seeded).expect("Falha ao criar SB15_14_seed.csv (vazio)");
        println!(
            "SB15_14_seed_{}.csv (vazio) salvo em '{}'",
            seed, out_path_seeded
        );
        let main_out_path = "output/SB15_14.csv";
        File::create(main_out_path).expect("Falha ao criar SB15_14.csv (vazio)");
        println!("SB15_14.csv (vazio) salvo em '{}'", main_out_path);
        return;
    }

    let mut solution = Vec::with_capacity(total_s14_to_cover / 15 + 1);
    let mut threshold = 15;
    let start_time = Instant::now();

    let mut total_s14_covered_count = 0u64;
    let barra = get_bar(total_s14_to_cover as u64);
    barra.set_message("Iniciando processo de cobertura para S14...");

    loop {
        let mut current_uncovered_s14 = original_uncovered_s14.clone();
        // Remove combinações já cobertas pela solução atual
        for &mask15 in &solution {
            let combo15_seq = mask_para_seq(mask15);
            // Cada S15 cobre 15 S14s. Iterar sobre os 15 números para formar as S14s.
            for i in 0..combo15_seq.len() {
                let mut s14_sub_seq = combo15_seq.clone();
                s14_sub_seq.remove(i); // Remove um número para formar uma S14
                let s14_sub_mask = seq_para_mask(&s14_sub_seq);
                current_uncovered_s14.remove(&s14_sub_mask);
            }
        }

        let remaining_s14_at_pass_start = current_uncovered_s14.len();

        barra.set_length(total_s14_to_cover as u64); // Ensure bar length is correct
        barra.set_position(total_s14_covered_count);
        barra.set_message(format!(
            "Threshold: {} | S14 Restantes: {} | S15 Solução: {}",
            threshold,
            remaining_s14_at_pass_start,
            solution.len()
        ));

        let mut s15_candidates: Vec<Vec<u8>> = (1u8..=25).combinations(15).collect();
        s15_candidates.shuffle(&mut rng);

        for combo15_seq_candidate in s15_candidates {
            let mask15_candidate = seq_para_mask(&combo15_seq_candidate);
            let mut current_s15_covers_how_many_new_s14 = 0;
            let mut s14_masks_covered_by_this_s15 = Vec::new();

            for i in 0..combo15_seq_candidate.len() {
                let mut s14_sub_seq = combo15_seq_candidate.clone();
                s14_sub_seq.remove(i);
                let s14_sub_mask = seq_para_mask(&s14_sub_seq);

                if current_uncovered_s14.contains(&s14_sub_mask) {
                    current_s15_covers_how_many_new_s14 += 1;
                    s14_masks_covered_by_this_s15.push(s14_sub_mask);
                }
            }

            if current_s15_covers_how_many_new_s14 >= threshold {
                for s14_mask in s14_masks_covered_by_this_s15 {
                    if current_uncovered_s14.remove(&s14_mask) {}
                }

                barra.inc(current_s15_covers_how_many_new_s14 as u64);
                total_s14_covered_count += current_s15_covers_how_many_new_s14 as u64;

                solution.push(mask15_candidate);

                let current_cobertura_percentual = if total_s14_to_cover > 0 {
                    (total_s14_covered_count as f64 / total_s14_to_cover as f64) * 100.0
                } else {
                    0.0
                };
                barra.set_message(format!(
                    "S15: {} | S14: {}/{} ({:.1}%) | Thr: {}",
                    solution.len(),
                    total_s14_covered_count,
                    total_s14_to_cover,
                    current_cobertura_percentual,
                    threshold
                ));

                if current_uncovered_s14.is_empty() {
                    break;
                }
            }
        }

        if current_uncovered_s14.is_empty() {
            barra.finish_with_message(format!(
                "Cobertura completa de S14 alcançada! {}/{} S14.",
                total_s14_covered_count, total_s14_to_cover
            ));
            break;
        } else if threshold > 1 {
            threshold -= 1;
        } else {
            barra.finish_with_message(format!(
                "Threshold mínimo (1) alcançado. Cobertura: {}/{} S14.",
                total_s14_covered_count, total_s14_to_cover
            ));
            break;
        }
    }

    if !barra.is_finished() {
        barra.finish_with_message(format!(
            "Processamento de S14 concluído. Cobertura final: {}/{} S14.",
            total_s14_covered_count, total_s14_to_cover
        ));
    }

    let elapsed = start_time.elapsed();
    println!(
        "Algoritmo para S14 concluído com {} S15 em {:.2?}.",
        solution.len(),
        elapsed
    );
    let final_coverage_percentage = if total_s14_to_cover > 0 {
        (total_s14_covered_count as f64 / total_s14_to_cover as f64) * 100.0
    } else {
        0.0
    };
    println!(
        "Cobertura final: {}/{} S14 ({:.2}%)",
        total_s14_covered_count, total_s14_to_cover, final_coverage_percentage
    );

    let out_path_seeded = format!("output/combinacoes/SB15_14_seed_{}.csv", seed);
    if let Err(e) = crate::common::salvar_solucao_csv(&out_path_seeded, &solution) {
        eprintln!("Erro escrevendo solução para SB15_14_seed.csv: {}", e);
    }
    println!("SB15_14 (seed {}) salvo em '{}'", seed, out_path_seeded);
}
