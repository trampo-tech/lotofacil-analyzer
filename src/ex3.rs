use crate::common::{carregar_combinacoes, get_bar, seq_para_mask, obter_seed};
use itertools::Itertools;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fs::{File, create_dir_all};
use std::time::Instant;

pub fn executar(seed_param: Option<u64>) {
    create_dir_all("output").expect("Não pôde criar diretório output");

    let seed = obter_seed(seed_param, "LOTOFACIL_SEED", "ex3");
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    println!("Carregando S13...");
    let original_s13_to_cover = carregar_combinacoes("output/saida_S13.csv", 5_200_300);
    let total_s13_to_cover_initially = original_s13_to_cover.len();
    println!(
        "S13 carregado: {} combinações a cobrir",
        total_s13_to_cover_initially
    );

    if total_s13_to_cover_initially == 0 {
        println!("Nenhuma combinação S13 para cobrir. Saindo.");
        let out_path = "output/SB15_13.csv";
        File::create(out_path).expect("Falha ao criar SB15_13.csv");
        println!("SB15_13.csv (vazio) salvo em '{}'", out_path);
        let out_path_seeded = format!("output/SB15_13_seed_{}.csv", seed);
        File::create(&out_path_seeded).expect("Falha ao criar SB15_13_seed.csv");
        println!(
            "SB15_13_seed_{}.csv (vazio) salvo em '{}'",
            seed, out_path_seeded
        );
        return;
    }

    let mut solution = Vec::new();
    let mut s13_usados = HashSet::new();
    let start_time = Instant::now();

    let barra = get_bar(total_s13_to_cover_initially as u64);

    println!();
    let mut todas_s15_seq: Vec<Vec<u8>> = (1u8..=25).combinations(15).collect();
    todas_s15_seq.shuffle(&mut rng);

    let remove2_indices: Vec<Vec<usize>> = (0..15).combinations(2).collect();

    for combo15_seq in todas_s15_seq {
        let mask15 = seq_para_mask(&combo15_seq);

        let mut s13s_desta_s15_set = HashSet::new();
        for rem_idx_pair in &remove2_indices {
            let mut s13_sub_mask = mask15;
            // combo15_seq[rem_idx_pair[0]] é o número no primeiro índice a remover
            // combo15_seq[rem_idx_pair[1]] é o número no segundo índice a remover
            s13_sub_mask &= !(1 << (combo15_seq[rem_idx_pair[0]] - 1));
            s13_sub_mask &= !(1 << (combo15_seq[rem_idx_pair[1]] - 1));

            if original_s13_to_cover.contains(&s13_sub_mask) {
                s13s_desta_s15_set.insert(s13_sub_mask);
            }
        }

        let novos_s13: HashSet<_> = s13s_desta_s15_set
            .difference(&s13_usados)
            .cloned()
            .collect();

        if !novos_s13.is_empty() {
            let similares = s13s_desta_s15_set.len() - novos_s13.len();
            let cobertura_percentual =
                (s13_usados.len() as f64 / total_s13_to_cover_initially as f64) * 100.0;

            let limite_adaptativo_s13 = if cobertura_percentual < 50.0 {
                35 // Início: muito seletivo
            } else if cobertura_percentual < 80.0 {
                56 // Meio: moderadamente seletivo
            } else if cobertura_percentual < 95.0 {
                84 // Final: menos seletivo
            } else {
                105 // Últimas S13: aceita qualquer contribuição
            };

            let contribuicao_significativa_s13 = novos_s13.len() >= 1
                && (cobertura_percentual > 90.0 || novos_s13.len() > similares / 2);

            if similares < limite_adaptativo_s13 || contribuicao_significativa_s13 {
                solution.push(mask15);
                s13_usados.extend(novos_s13.iter());

                barra.inc(novos_s13.len() as u64);
                let current_cobertura_percentual =
                    (s13_usados.len() as f64 / total_s13_to_cover_initially as f64) * 100.0;
                barra.set_message(format!(
                    "S15: {} | S13: {}/{} ({:.1}%)",
                    solution.len(),
                    s13_usados.len(),
                    total_s13_to_cover_initially,
                    current_cobertura_percentual
                ));
            }
        }

        if s13_usados.len() >= total_s13_to_cover_initially {
            barra.finish_with_message(format!(
                "Cobertura completa de S13 alcançada! {}/{} S13.",
                s13_usados.len(),
                total_s13_to_cover_initially
            ));
            break;
        }
    }
    let elapsed = start_time.elapsed();

    if s13_usados.len() >= total_s13_to_cover_initially {
        println!(
            "Algoritmo para S13 concluído com 100% de cobertura, usando {} S15 em {:.2?}.",
            solution.len(),
            elapsed
        );

        let out_path_seeded = format!("output/combinacoes/SB15_13_seed_{}.csv", seed);
        if let Err(e) = crate::common::salvar_solucao_csv(&out_path_seeded, &solution) {
            eprintln!(
                "Erro escrevendo solução para output/SB15_13_seed_{}.csv: {}",
                seed, e
            );
        }
        println!(
            "Solução SB15_13 (seed {}) com 100% cobertura salva em '{}'",
            seed, out_path_seeded
        );
    } else {
        let cobertura_percentual_final =
            (s13_usados.len() as f64 / total_s13_to_cover_initially as f64) * 100.0;
        println!(
            "Algoritmo para S13 NÃO atingiu 100% de cobertura após {:.2?}.",
            elapsed
        );
        println!(
            "Cobertura final: {}/{} ({:.1}%) com {} S15 selecionadas.",
            s13_usados.len(),
            total_s13_to_cover_initially,
            cobertura_percentual_final,
            solution.len()
        );
        println!(
            "Nenhum arquivo de solução output/SB15_13_seed_{}.csv foi salvo pois a cobertura não foi total.",
            seed
        );
    }
}
