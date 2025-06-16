use crate::common::{mask_para_seq, otimizar_solucao_completa, seq_para_mask, carregar_combinacoes, get_bar};
use itertools::Itertools;
use std::fs::{File, create_dir_all};
use std::io::{BufWriter, Write};
use std::time::Instant;

pub fn executar() {
    create_dir_all("output").expect("Não pôde criar output");

    println!("Carregando S14...");
    let original_uncovered = carregar_combinacoes("output/saida_S14.csv",4_500_000);
    let total_s14 = original_uncovered.len();
    println!("S14 carregado: {} combinações", total_s14);

    let mut solution = Vec::with_capacity(total_s14 / 15 + 1);
    let mut threshold = 15;
    let start = Instant::now();

    let mut total_covered = 0u64;
    let barra = get_bar(total_s14 as u64);

    loop {
        let mut uncovered = original_uncovered.clone();
        // Remove combinações já cobertas pela solução atual
        for &mask15 in &solution {
            let combo = mask_para_seq(mask15);
            for &n in &combo {
                let sub = mask15 & !(1 << (n - 1));
                uncovered.remove(&sub);
            }
        }

        let remaining_at_start = uncovered.len();
        let mut found_in_this_pass = 0;
        let mut covered_in_this_pass = 0u64;
        
        // Update the persistent progress bar for this threshold
        barra.set_length(total_s14 as u64);
        barra.set_position(total_covered);
        barra.set_message(format!("Threshold: {} | Restam: {}", threshold, remaining_at_start));

        for combo in (1u8..=25).combinations(15) {
            let mask15 = seq_para_mask(&combo);
            let mut covered = 0;
            let mut covered_masks = Vec::new();

            for &n in &combo {
                let sub = mask15 & !(1 << (n - 1));
                if uncovered.contains(&sub) {
                    covered += 1;
                    covered_masks.push(sub);
                }
            }

            if covered >= threshold {
                // Remove as combinações cobertas
                for mask in covered_masks {
                    uncovered.remove(&mask);
                }
                barra.inc(covered as u64);
                covered_in_this_pass += covered as u64;
                total_covered += covered as u64;

                solution.push(mask15);
                found_in_this_pass += 1;

                if uncovered.is_empty() {
                    println!("Cobertura completa alcançada!");
                    break;
                }
            }
        }

        barra.set_message(format!(
            "Threshold {}: {} S15 encontrados, {} combinações cobertas", 
            threshold, found_in_this_pass, covered_in_this_pass
        ));


        if uncovered.is_empty() {
            break;
        } else if threshold > 1 {
            threshold -= 1;
            
        } else {
            barra.finish_with_message("Threshold mínimo alcançado, mas cobertura incompleta.");
            break;
        }
    }

    barra.finish_with_message("Processamento inicial concluído");

    let elapsed = start.elapsed();
    println!(
        "Algoritmo inicial concluído com {} S15 em {:.2?}",
        solution.len(),
        elapsed
    );

    // FASE DE OTIMIZAÇÃO
    let (initial_size, final_size, opt_elapsed) =
        otimizar_solucao_completa(&mut solution, &original_uncovered);

    println!(
        "Processo completo finalizado: {} S15 (inicial: {}) em {:.2?} total",
        final_size,
        initial_size,
        elapsed + opt_elapsed
    );

    let out = File::create("output/SB15_14.csv").expect("Falha ao criar SB15_14.csv");
    let mut writer = BufWriter::new(out);
    for &mask in &solution {
        let seq = mask_para_seq(mask);
        let line = seq
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        writeln!(writer, "{}", line).expect("Erro escrevendo solução");
    }
    println!("SB15_14 salvo em 'output/SB15_14.csv'");
}
