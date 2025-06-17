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
                    println!("Seed gerada para ex4: {}", random_seed);
                }
                random_seed
            })
    });

    if seed_param.is_some() {
        // println!("Usando seed fornecida para ex4: {}", seed);
    } else if std::env::var("LOTOFACIL_SEED").is_ok() {
        println!("Usando seed específica do ENV para ex4: {}", seed);
    }

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    println!("Carregando S12...");
    let original_s12_to_cover = carregar_combinacoes("output/saida_S12.csv", 1_600_000); // Max S12 C(25,12) = 5_200_300, mas na prática menos
    let total_s12_to_cover_initially = original_s12_to_cover.len();
    println!(
        "S12 carregado: {} combinações a cobrir",
        total_s12_to_cover_initially
    );

    if total_s12_to_cover_initially == 0 {
        println!("Nenhuma combinação S12 para cobrir. Saindo.");
        let out_path = "output/SB15_12.csv";
        File::create(out_path).expect("Falha ao criar SB15_12.csv");
        println!("SB15_12.csv (vazio) salvo em '{}'", out_path);
        let out_path_seeded = format!("output/SB15_12_seed_{}.csv", seed);
        File::create(&out_path_seeded).expect("Falha ao criar SB15_12_seed.csv");
        println!(
            "SB15_12_seed_{}.csv (vazio) salvo em '{}'",
            seed, out_path_seeded
        );
        return;
    }

    let mut solution = Vec::new();
    let mut s12_usados = HashSet::new(); // Renomeado de 'uncovered' e tipo alterado
    let start_time = Instant::now();

    // Uso direto da barra de progresso
    let barra = get_bar(total_s12_to_cover_initially as u64);
    barra.set_message("Processando combinações S15 para cobrir S12s...");

    println!("Gerando e embaralhando combinações S15...");
    let mut todas_s15_seq: Vec<Vec<u8>> = (1u8..=25).combinations(15).collect();
    todas_s15_seq.shuffle(&mut rng);
    println!(
        "Total de combinações S15: {} (ordem randomizada)",
        todas_s15_seq.len()
    );

    // Auxiliar para gerar S12s de uma S15: índices para remover 3 números
    let remove3_indices: Vec<Vec<usize>> = (0..15).combinations(3).collect();

    // barra.set_message é chamado acima quando a barra é criada.

    for combo15_seq in todas_s15_seq {
        let mask15 = seq_para_mask(&combo15_seq);

        let mut s12s_desta_s15_set = HashSet::new();
        for rem_idx_pair in &remove3_indices {
            let mut s12_sub_mask = mask15;
            for &idx_in_combo15 in rem_idx_pair {
                s12_sub_mask &= !(1 << (combo15_seq[idx_in_combo15] - 1));
            }

            if original_s12_to_cover.contains(&s12_sub_mask) {
                s12s_desta_s15_set.insert(s12_sub_mask);
            }
        }

        let novos_s12: HashSet<_> = s12s_desta_s15_set
            .difference(&s12_usados)
            .cloned()
            .collect();

        if !novos_s12.is_empty() {
            let similares = s12s_desta_s15_set.len() - novos_s12.len();
            let cobertura_percentual =
                (s12_usados.len() as f64 / total_s12_to_cover_initially as f64) * 100.0;

            let limite_adaptativo_s12 = if cobertura_percentual < 50.0 {
                152 // Início: muito seletivo
            } else if cobertura_percentual < 80.0 {
                243 // Meio: moderadamente seletivo
            } else if cobertura_percentual < 95.0 {
                364 // Final: menos seletivo
            } else {
                455 // Últimas S12: aceita qualquer contribuição
            };

            let contribuicao_significativa_s12 = novos_s12.len() >= 1
                && (cobertura_percentual > 90.0 || novos_s12.len() > similares / 2);

            if similares < limite_adaptativo_s12 || contribuicao_significativa_s12 {
                solution.push(mask15);
                s12_usados.extend(novos_s12.iter());

                // Uso direto da barra
                barra.inc(novos_s12.len() as u64);
                let current_cobertura_percentual =
                    (s12_usados.len() as f64 / total_s12_to_cover_initially as f64) * 100.0;
                barra.set_message(format!(
                    "S15: {} | S12: {}/{} ({:.1}%)",
                    solution.len(),
                    s12_usados.len(),
                    total_s12_to_cover_initially,
                    current_cobertura_percentual
                ));
            }
        }

        if s12_usados.len() >= total_s12_to_cover_initially {
            // Uso direto da barra
            barra.finish_with_message(format!(
                "Cobertura completa de S12 alcançada! {}/{} S12.",
                s12_usados.len(),
                total_s12_to_cover_initially
            ));
            break;
        }
    }

    // Após o loop, trata o término da barra se não foi finalizada pelo break
    // Uso direto da barra
    if !barra.is_finished() {
        if s12_usados.len() < total_s12_to_cover_initially && !solution.is_empty() {
            barra.finish_with_message(format!(
                "Processamento de S15 concluído. Cobertura: {}/{} S12.",
                s12_usados.len(),
                total_s12_to_cover_initially
            ));
        } else if solution.is_empty() && total_s12_to_cover_initially > 0 {
            barra.finish_with_message(format!(
                "Nenhuma combinação S15 encontrada para S12. {}/{} S12 cobertos.",
                s12_usados.len(),
                total_s12_to_cover_initially
            ));
        } else if total_s12_to_cover_initially == 0 {
            barra.finish_with_message("Nenhuma combinação S12 para cobrir.");
        }
    }

    let elapsed = start_time.elapsed();
    println!(
        "Algoritmo para S12 concluído com {} S15 em {:.2?}.",
        solution.len(),
        elapsed
    );
    println!(
        "Cobertura final: {}/{} S12 ({:.2}%)",
        s12_usados.len(),
        total_s12_to_cover_initially,
        (s12_usados.len() as f64 / total_s12_to_cover_initially as f64) * 100.0
    );

    let out_path_seeded = format!("output/SB15_12_seed_{}.csv", seed);
    let out_file_seeded =
        File::create(&out_path_seeded).expect("Falha ao criar arquivo SB15_12 com seed");
    let mut writer_seeded = BufWriter::new(out_file_seeded);
    for &mask in &solution {
        let seq = mask_para_seq(mask);
        let line = seq
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        writeln!(writer_seeded, "{}", line).expect("Erro escrevendo solução para SB15_12_seed.csv");
    }
    println!("SB15_12 (seed {}) salvo em '{}'", seed, out_path_seeded);

    let main_out_path = "output/SB15_12.csv";
    let main_out_file = File::create(main_out_path).expect("Falha ao criar SB15_12.csv");
    let mut main_writer = BufWriter::new(main_out_file);
    for &mask in &solution {
        let seq = mask_para_seq(mask);
        let line = seq
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        writeln!(main_writer, "{}", line).expect("Erro escrevendo solução para SB15_12.csv");
    }
    println!("Cópia também salva em '{}'", main_out_path);
}
