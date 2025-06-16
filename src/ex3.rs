use crate::common::{
    carregar_combinacoes, get_bar, mask_para_seq, otimizar_solucao_completa, seq_para_mask,
};
use itertools::Itertools;
use std::fs::{File, create_dir_all};
use std::io::{BufWriter, Write};
use std::time::Instant;


pub fn executar() {
    create_dir_all("output").expect("Não pôde criar diretório output");

    println!("Carregando S13...");
    // This set contains all S13 combinations that need to be covered.
    let original_s13_to_cover = carregar_combinacoes("output/saida_S13.csv", 5_200_300);
    let total_s13_to_cover_initially = original_s13_to_cover.len();
    println!(
        "S13 carregado: {} combinações a cobrir",
        total_s13_to_cover_initially
    );

    if total_s13_to_cover_initially == 0 {
        println!("Nenhuma combinação S13 para cobrir. Saindo.");
        // Optionally create an empty SB15_13.csv file
        let out_path = "output/SB15_13.csv";
        File::create(out_path).expect("Falha ao criar SB15_13.csv");
        println!("SB15_13.csv (vazio) salvo em '{}'", out_path);
        return;
    }

    let mut solution = Vec::with_capacity(total_s13_to_cover_initially / 105 + 100); // C(15,2) = 105 S13s per S15
    let mut threshold = 105; // Max S13s a S15 can cover C(15,2)
    let start_time = Instant::now();

    let mut total_s13_actually_covered_count = 0u64;
    let barra = get_bar(total_s13_to_cover_initially as u64);

    // Indices to remove 2 numbers from a 15-number combination to get a 13-number combination
    let remove2_indices = (0..15).combinations(2).collect::<Vec<_>>();

    'threshold_loop: loop {
        // This set represents S13s that are not yet covered by the 'solution' found so far.
        // It's recalculated at the start of each threshold pass.
        let mut s13_still_needing_coverage_this_pass = original_s13_to_cover.clone();
        for &mask15_in_solution in &solution {
            let combo15_from_solution = mask_para_seq(mask15_in_solution);
            for rem_idx_pair in &remove2_indices {
                let mut s13_sub_mask = mask15_in_solution;
                s13_sub_mask &= !(1 << (combo15_from_solution[rem_idx_pair[0]] - 1));
                s13_sub_mask &= !(1 << (combo15_from_solution[rem_idx_pair[1]] - 1));
                s13_still_needing_coverage_this_pass.remove(&s13_sub_mask);
            }
        }

        let remaining_s13_at_start_of_pass = s13_still_needing_coverage_this_pass.len();
        
        if remaining_s13_at_start_of_pass == 0 {
             // This check ensures that if previous thresholds already covered everything, we stop.
            println!("Cobertura completa de S13 já alcançada antes de processar threshold {}.", threshold);
            break 'threshold_loop;
        }

        let mut s15_found_in_this_pass = 0;
        let mut s13_covered_in_this_pass_count = 0u64;

        barra.set_length(total_s13_to_cover_initially as u64); // Total S13s to cover
        barra.set_position(total_s13_actually_covered_count);   // S13s covered by solution so far
        barra.set_message(format!(
            "S13 | Threshold: {} | Restam: {} (de {})",
            threshold, remaining_s13_at_start_of_pass, total_s13_to_cover_initially
        ));

        for combo15_potential_seq in (1u8..=25).combinations(15) {
            let m15_potential = seq_para_mask(&combo15_potential_seq);
            let mut count_s13_covered_by_this_m15 = 0;
            let mut s13_masks_list_covered_by_this_m15 = Vec::new();

            for rem_idx_pair in &remove2_indices {
                let mut sub_mask_s13 = m15_potential;
                sub_mask_s13 &= !(1 << (combo15_potential_seq[rem_idx_pair[0]] - 1));
                sub_mask_s13 &= !(1 << (combo15_potential_seq[rem_idx_pair[1]] - 1));

                if s13_still_needing_coverage_this_pass.contains(&sub_mask_s13) {
                    count_s13_covered_by_this_m15 += 1;
                    s13_masks_list_covered_by_this_m15.push(sub_mask_s13);
                }
            }

            if count_s13_covered_by_this_m15 >= threshold {
                solution.push(m15_potential);
                s15_found_in_this_pass += 1;
                
                let mut newly_covered_s13_by_this_m15_this_pass = 0u64;
                for s13_mask_to_remove in s13_masks_list_covered_by_this_m15 {
                    // Remove from the pass-local set to avoid re-covering by subsequent S15s in *this same pass*
                    if s13_still_needing_coverage_this_pass.remove(&s13_mask_to_remove) {
                        newly_covered_s13_by_this_m15_this_pass += 1;
                    }
                }
                
                barra.inc(newly_covered_s13_by_this_m15_this_pass);
                s13_covered_in_this_pass_count += newly_covered_s13_by_this_m15_this_pass;
                total_s13_actually_covered_count += newly_covered_s13_by_this_m15_this_pass;

                if s13_still_needing_coverage_this_pass.is_empty() {
                    println!("Cobertura completa de S13 alcançada durante threshold {}!", threshold);
                    // This break is for the inner C(25,15) loop
                    break; 
                }
            }
        }

        barra.set_message(format!(
            "S13 | Threshold {}: {} S15 adicionados, {} S13 cobertos. Total S13 cobertos: {}/{}",
            threshold, s15_found_in_this_pass, s13_covered_in_this_pass_count, total_s13_actually_covered_count, total_s13_to_cover_initially
        ));

        if s13_still_needing_coverage_this_pass.is_empty() || total_s13_actually_covered_count >= total_s13_to_cover_initially as u64 {
            break 'threshold_loop; // All S13s are covered
        }

        if threshold > 1 {
            threshold -= 1;
        } else {
            barra.finish_with_message(format!(
                "S13 | Threshold mínimo alcançado. Cobertura incompleta: {}/{} S13.",
                 total_s13_actually_covered_count, total_s13_to_cover_initially
            ));
            break 'threshold_loop;
        }
    }

    if total_s13_actually_covered_count >= total_s13_to_cover_initially as u64 {
        barra.finish_with_message(format!(
            "Cobertura completa de S13 alcançada! {}/{} S13.",
            total_s13_actually_covered_count, total_s13_to_cover_initially
        ));
    } else if solution.is_empty() && total_s13_to_cover_initially > 0 {
         barra.finish_with_message(format!("Nenhuma combinação S15 encontrada para S13. {}/{} S13 cobertos.", total_s13_actually_covered_count, total_s13_to_cover_initially));
    }


    let elapsed = start_time.elapsed();
    println!(
        "Processo para S13 concluído com {} S15 em {:.2?}. {}/{} S13 cobertos.",
        solution.len(),
        elapsed,
        total_s13_actually_covered_count,
        total_s13_to_cover_initially
    );

    let out_path = "output/SB15_13.csv";
    let out_file = File::create(out_path).expect("Falha ao criar SB15_13.csv");
    let mut writer = BufWriter::new(out_file);
    for &mask in &solution {
        let seq = mask_para_seq(mask);
        let line = seq
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        writeln!(writer, "{}", line).expect("Erro escrevendo solução para SB15_13.csv");
    }
    println!("SB15_13 salvo em '{}'", out_path);
}
