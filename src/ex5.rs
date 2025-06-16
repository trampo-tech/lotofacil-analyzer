use crate::common::{
    carregar_combinacoes, get_bar, mask_para_seq, otimizar_solucao_completa, seq_para_mask,
};
use itertools::Itertools;
use std::fs::{File, create_dir_all};
use std::io::{BufWriter, Write};
use std::time::Instant;



pub fn executar() {
    create_dir_all("output").expect("Não pôde criar diretório output");
    println!("Carregando S11...");
    let mut uncovered = carregar_combinacoes("output/saida_S11.csv", 4_457_400);
    let total_s11 = uncovered.len();
    println!("S11 carregado: {} combinações a cobrir", total_s11);
    
    let mut solution = Vec::with_capacity(total_s11 / 1365 + 1); // C(15,4) = 1365
    let start = Instant::now();

    let barra = get_bar(total_s11 as u64);
    barra.set_message(format!("S11: {} restantes", uncovered.len()));

    let remove4_indices = (0..15).combinations(4).collect::<Vec<_>>();

    for combo15_seq in (1u8..=25).combinations(15) {
        let m15 = seq_para_mask(&combo15_seq);
        let mut newly_covered_this_pass = 0;

        for rem_indices in &remove4_indices {
            let mut sub_mask = m15;
            for &idx_in_combo15 in rem_indices {
                sub_mask &= !(1 << (combo15_seq[idx_in_combo15] - 1));
            }
            if uncovered.remove(&sub_mask) {
                newly_covered_this_pass += 1;
            }
        }

        if newly_covered_this_pass > 0 {
            solution.push(m15);
            barra.inc(newly_covered_this_pass as u64);
            barra.set_message(format!("S11: {} restantes", uncovered.len()));
            if uncovered.is_empty() {
                barra.finish_with_message("Cobertura completa de S11 alcançada!");
                break;
            }
        }
    }

    if !uncovered.is_empty() {
        barra.finish_with_message(format!(
            "Processamento de S11 concluído. {} S11 restantes.",
            uncovered.len()
        ));
    } else if solution.is_empty() && total_s11 > 0 {
        barra.finish_with_message("Nenhuma combinação S15 necessária ou encontrada para S11.");
    } else if total_s11 == 0 {
        barra.finish_with_message("Nenhuma combinação S11 para cobrir.");
    }


    let elapsed = start.elapsed();
    println!(
        "Cobertura de S11 concluída com {} S15 em {:.2?}",
        solution.len(),
        elapsed
    );

    let out_path = "output/SB15_11.csv";
    let out_file = File::create(out_path).expect("Falha ao criar SB15_11.csv");
    let mut writer = BufWriter::new(out_file);
    for &mask in &solution {
        let seq = mask_para_seq(mask);
        let line = seq
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        writeln!(writer, "{}", line).expect("Erro escrevendo solução para SB15_11.csv");
    }
    println!("SB15_11 salvo em '{}'", out_path);
}
