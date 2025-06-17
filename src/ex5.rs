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
                    println!("Seed gerada para ex5: {}", random_seed);
                }
                random_seed
            })
    });

    if seed_param.is_some() {
        // println!("Usando seed fornecida para ex5: {}", seed);
    } else if std::env::var("LOTOFACIL_SEED").is_ok() {
        println!("Usando seed específica do ENV para ex5: {}", seed);
    }

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    println!("Carregando S11...");
    // Max S11 C(25,11) = 4_457_400. Capacidade ajustada para carregar_combinacoes se necessário.
    let original_s11_to_cover = carregar_combinacoes("output/saida_S11.csv", 4_457_400);
    let total_s11_to_cover_initially = original_s11_to_cover.len();
    println!(
        "S11 carregado: {} combinações a cobrir",
        total_s11_to_cover_initially
    );

    if total_s11_to_cover_initially == 0 {
        println!("Nenhuma combinação S11 para cobrir. Saindo.");
        let out_path = "output/SB15_11.csv";
        File::create(out_path).expect("Falha ao criar SB15_11.csv");
        println!("SB15_11.csv (vazio) salvo em '{}'", out_path);
        let out_path_seeded = format!("output/SB15_11_seed_{}.csv", seed);
        File::create(&out_path_seeded).expect("Falha ao criar SB15_11_seed.csv");
        println!(
            "SB15_11_seed_{}.csv (vazio) salvo em '{}'",
            seed, out_path_seeded
        );
        return;
    }

    let mut solution = Vec::new();
    let mut s11_usados = HashSet::new();
    let start_time = Instant::now();

    // Uso direto da barra de progresso
    let barra = get_bar(total_s11_to_cover_initially as u64);
    barra.set_message("Processando combinações S15 para cobrir S11s...");

    println!("Gerando e embaralhando combinações S15...");
    let mut todas_s15_seq: Vec<Vec<u8>> = (1u8..=25).combinations(15).collect();
    todas_s15_seq.shuffle(&mut rng);
    println!(
        "Total de combinações S15: {} (ordem randomizada)",
        todas_s15_seq.len()
    );

    // Auxiliar para gerar S11s de uma S15: índices para remover 4 números
    let remove4_indices: Vec<Vec<usize>> = (0..15).combinations(4).collect();

    // barra.set_message é chamado acima quando a barra é criada.

    for combo15_seq in todas_s15_seq {
        let mask15 = seq_para_mask(&combo15_seq);

        let mut s11s_desta_s15_set = HashSet::new();
        for rem_indices_group in &remove4_indices {
            let mut s11_sub_mask = mask15;
            for &idx_in_combo15 in rem_indices_group {
                s11_sub_mask &= !(1 << (combo15_seq[idx_in_combo15] - 1));
            }

            if original_s11_to_cover.contains(&s11_sub_mask) {
                s11s_desta_s15_set.insert(s11_sub_mask);
            }
        }

        let novos_s11: HashSet<_> = s11s_desta_s15_set
            .difference(&s11_usados)
            .cloned()
            .collect();

        if !novos_s11.is_empty() {
            let similares = s11s_desta_s15_set.len() - novos_s11.len();
            let cobertura_percentual =
                (s11_usados.len() as f64 / total_s11_to_cover_initially as f64) * 100.0;

            let limite_adaptativo_s11 = if cobertura_percentual < 50.0 {
                455 // Início: muito seletivo
            } else if cobertura_percentual < 80.0 {
                728 // Meio: moderadamente seletivo
            } else if cobertura_percentual < 95.0 {
                1092 // Final: menos seletivo
            } else {
                1365 // Últimas S11: aceita qualquer contribuição
            };

            let contribuicao_significativa_s11 = novos_s11.len() >= 1
                && (cobertura_percentual > 90.0 || novos_s11.len() > similares / 2);

            if similares < limite_adaptativo_s11 || contribuicao_significativa_s11 {
                solution.push(mask15);
                s11_usados.extend(novos_s11.iter());

                // Uso direto da barra
                barra.inc(novos_s11.len() as u64);
                let current_cobertura_percentual =
                    (s11_usados.len() as f64 / total_s11_to_cover_initially as f64) * 100.0;
                barra.set_message(format!(
                    "S15: {} | S11: {}/{} ({:.1}%)",
                    solution.len(),
                    s11_usados.len(),
                    total_s11_to_cover_initially,
                    current_cobertura_percentual
                ));
            }
        }

        if s11_usados.len() >= total_s11_to_cover_initially {
            // Uso direto da barra
            barra.finish_with_message(format!(
                "Cobertura completa de S11 alcançada! {}/{} S11.",
                s11_usados.len(),
                total_s11_to_cover_initially
            ));
            break;
        }
    }

    // Após o loop, trata o término da barra se não foi finalizada pelo break
    // Uso direto da barra
    if !barra.is_finished() {
        if s11_usados.len() < total_s11_to_cover_initially && !solution.is_empty() {
            barra.finish_with_message(format!(
                "Processamento de S15 concluído. Cobertura: {}/{} S11.",
                s11_usados.len(),
                total_s11_to_cover_initially
            ));
        } else if solution.is_empty() && total_s11_to_cover_initially > 0 {
            barra.finish_with_message(format!(
                "Nenhuma combinação S15 encontrada para S11. {}/{} S11 cobertos.",
                s11_usados.len(),
                total_s11_to_cover_initially
            ));
        } else if total_s11_to_cover_initially == 0 {
            barra.finish_with_message("Nenhuma combinação S11 para cobrir.");
        }
    }

    let elapsed = start_time.elapsed();
    println!(
        "Algoritmo para S11 concluído com {} S15 em {:.2?}.",
        solution.len(),
        elapsed
    );
    println!(
        "Cobertura final: {}/{} S11 ({:.2}%)",
        s11_usados.len(),
        total_s11_to_cover_initially,
        (s11_usados.len() as f64 / total_s11_to_cover_initially as f64) * 100.0
    );

    let out_path_seeded = format!("output/SB15_11_seed_{}.csv", seed);
    let out_file_seeded =
        File::create(&out_path_seeded).expect("Falha ao criar arquivo SB15_11 com seed");
    let mut writer_seeded = BufWriter::new(out_file_seeded);
    for &mask in &solution {
        let seq = mask_para_seq(mask);
        let line = seq
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        writeln!(writer_seeded, "{}", line).expect("Erro escrevendo solução para SB15_11_seed.csv");
    }
    println!("SB15_11 (seed {}) salvo em '{}'", seed, out_path_seeded);

    let main_out_path = "output/SB15_11.csv";
    let main_out_file = File::create(main_out_path).expect("Falha ao criar SB15_11.csv");
    let mut main_writer = BufWriter::new(main_out_file);
    for &mask in &solution {
        let seq = mask_para_seq(mask);
        let line = seq
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        writeln!(main_writer, "{}", line).expect("Erro escrevendo solução para SB15_11.csv");
    }
    println!("Cópia também salva em '{}'", main_out_path);
}
