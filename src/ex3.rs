use crate::common::{carregar_combinacoes, get_bar, mask_para_seq, seq_para_mask};
use itertools::Itertools;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fs::{File, create_dir_all};
use std::io::{BufWriter, Write};
use std::time::Instant;

pub fn executar(seed_param: Option<u64>) {
    create_dir_all("output").expect("Não pôde criar diretório output");

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
                    println!("Seed gerada para ex3: {}", random_seed);
                }
                random_seed
            })
    });

    if seed_param.is_some() {
    } else if std::env::var("LOTOFACIL_SEED").is_ok() {
        println!("Usando seed específica do ENV para ex3: {}", seed);
    }

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

    // Uso direto da barra de progresso
    let barra = get_bar(total_s13_to_cover_initially as u64);
    barra.set_message("Processando combinações S15 para cobrir S13s...");

    println!();
    let mut todas_s15_seq: Vec<Vec<u8>> = (1u8..=25).combinations(15).collect();
    todas_s15_seq.shuffle(&mut rng);
    println!(
        "Total de combinações S15: {} (ordem randomizada)",
        todas_s15_seq.len()
    );

    // Auxiliar para gerar S13s de uma S15: índices para remover 2 números de uma combinação de 15 números
    let remove2_indices: Vec<Vec<usize>> = (0..15).combinations(2).collect();

    // barra.set_message é chamado acima quando a barra é criada.

    for combo15_seq in todas_s15_seq {
        let mask15 = seq_para_mask(&combo15_seq);

        // Gera todas as S13s que esta S15 pode cobrir do conjunto original_s13_to_cover
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

                // Uso direto da barra
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
            // Uso direto da barra
            barra.finish_with_message(format!(
                "Cobertura completa de S13 alcançada! {}/{} S13.",
                s13_usados.len(),
                total_s13_to_cover_initially
            ));
            break;
        }
    }

    // Após o loop, trata o término da barra se não foi finalizada pelo break
    // Uso direto da barra
    if !barra.is_finished() {
        if s13_usados.len() < total_s13_to_cover_initially && !solution.is_empty() {
            barra.finish_with_message(format!(
                "Processamento de S15 concluído. Cobertura: {}/{} S13.",
                s13_usados.len(),
                total_s13_to_cover_initially
            ));
        } else if solution.is_empty() && total_s13_to_cover_initially > 0 {
            barra.finish_with_message(format!(
                "Nenhuma combinação S15 encontrada para S13. {}/{} S13 cobertos.",
                s13_usados.len(),
                total_s13_to_cover_initially
            ));
        }
    }

    let elapsed = start_time.elapsed();
    println!(
        "Algoritmo para S13 concluído com {} S15 em {:.2?}.",
        solution.len(),
        elapsed
    );
    println!(
        "Cobertura final: {}/{} S13 ({:.2}%)",
        s13_usados.len(),
        total_s13_to_cover_initially,
        (s13_usados.len() as f64 / total_s13_to_cover_initially as f64) * 100.0
    );

    let out_path_seeded = format!("output/SB15_13_seed_{}.csv", seed);
    let out_file_seeded =
        File::create(&out_path_seeded).expect("Falha ao criar arquivo SB15_13 com seed");
    let mut writer_seeded = BufWriter::new(out_file_seeded);
    for &mask in &solution {
        let seq = mask_para_seq(mask);
        let line = seq
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        writeln!(writer_seeded, "{}", line).expect("Erro escrevendo solução para SB15_13_seed.csv");
    }
    println!("SB15_13 (seed {}) salvo em '{}'", seed, out_path_seeded);

    let main_out_path = "output/SB15_13.csv";
    let main_out_file = File::create(main_out_path).expect("Falha ao criar SB15_13.csv");
    let mut main_writer = BufWriter::new(main_out_file);
    for &mask in &solution {
        let seq = mask_para_seq(mask);
        let line = seq
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        writeln!(main_writer, "{}", line).expect("Erro escrevendo solução para SB15_13.csv");
    }
    println!("Cópia também salva em '{}'", main_out_path);
}
