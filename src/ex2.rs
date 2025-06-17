use crate::common::{carregar_combinacoes, get_bar, mask_para_seq, seq_para_mask};
use itertools::Itertools;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fs::{File, create_dir_all};
use std::io::{BufWriter, Write};
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
                // Não imprime a geração de seed aqui se chamado pelo otimizador (removido)
                if seed_param.is_none() && std::env::var("LOTOFACIL_SEED").is_err() {
                    println!("Seed gerada para ex2: {}", random_seed);
                }
                random_seed
            })
    });

    if seed_param.is_some() {
        // Suprimir potencialmente se chamado pelo otimizador para reduzir o ruído (removido)
        // println!("Usando seed fornecida para ex2: {}", seed);
    } else if std::env::var("LOTOFACIL_SEED").is_ok() {
        println!("Usando seed específica do ENV para ex2: {}", seed);
    }

    println!("Carregando S14...");
    let todas_s14 = carregar_combinacoes("output/saida_S14.csv", 4_500_000);
    let total_s14 = todas_s14.len();
    println!("S14 carregado: {} combinações", total_s14);

    if total_s14 == 0 {
        println!("Nenhuma combinação S14 para cobrir. Saindo.");
        let out_path = "output/SB15_14.csv";
        File::create(out_path).expect("Falha ao criar SB15_14.csv");
        println!("SB15_14.csv (vazio) salvo em '{}'", out_path);
        let out_path_seeded = format!("output/SB15_14_seed_{}.csv", seed);
        File::create(&out_path_seeded).expect("Falha ao criar SB15_14_seed.csv");
        println!(
            "SB15_14_seed_{}.csv (vazio) salvo em '{}'",
            seed, out_path_seeded
        );
        return;
    }

    let mut sb15_14 = Vec::new();
    let mut s14_usados = HashSet::new();

    let start = Instant::now();

    // Uso direto da barra de progresso
    let barra = get_bar(total_s14 as u64);
    barra.set_message("Gerando e embaralhando combinações S15...");
    let mut todas_s15: Vec<Vec<u8>> = (1u8..=25).combinations(15).collect();

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    todas_s15.shuffle(&mut rng);
    barra.set_message(format!("Total de combinações S15: {} (ordem randomizada)",todas_s15.len()));


    for combo15 in todas_s15 {
        let mask15 = seq_para_mask(&combo15);

        let mut s14s_desta_s15 = HashSet::new();
        for &numero in &combo15 {
            let s14_mask = mask15 & !(1 << (numero - 1));
            if todas_s14.contains(&s14_mask) {
                s14s_desta_s15.insert(s14_mask);
            }
        }

        let novos_s14: HashSet<_> = s14s_desta_s15.difference(&s14_usados).cloned().collect();

        if !novos_s14.is_empty() {
            let similares = s14s_desta_s15.len() - novos_s14.len();

            let cobertura_percentual_para_heuristica =
                (s14_usados.len() as f64 / total_s14 as f64) * 100.0;
            let limite_adaptativo = if cobertura_percentual_para_heuristica < 50.0 {
                5 // Início: muito seletivo
            } else if cobertura_percentual_para_heuristica < 80.0 {
                8 // Meio: moderadamente seletivo
            } else if cobertura_percentual_para_heuristica < 95.0 {
                12 // Final: menos seletivo
            } else {
                15 // Últimas S14: aceita qualquer contribuição
            };

            // Só aceita a S15 se a similaridade for baixa OU se contribuir significativamente
            let contribuicao_significativa = novos_s14.len() >= 1
                && (cobertura_percentual_para_heuristica > 90.0 || novos_s14.len() > similares / 2);

            if similares < limite_adaptativo || contribuicao_significativa {
                sb15_14.push(mask15);
                s14_usados.extend(novos_s14.iter());

                barra.inc(novos_s14.len() as u64);
                let current_coverage_for_msg = (s14_usados.len() as f64 / total_s14 as f64) * 100.0;
                barra.set_message(format!(
                    "S15: {} | S14: {}/{} ({:.1}%)",
                    sb15_14.len(),
                    s14_usados.len(),
                    total_s14,
                    current_coverage_for_msg
                ));
            }
        }

        if s14_usados.len() >= total_s14 {
            barra.finish_with_message("Cobertura completa de S14 alcançada!");
            break;
        }
    }

    // Após o loop, trata o término da barra se não foi finalizada pelo break
    if !barra.is_finished() {
        let final_coverage_percent = (s14_usados.len() as f64 / total_s14 as f64) * 100.0;
        barra.finish_with_message(format!(
            "Processamento de S15s concluído. Cobertura S14: {}/{} ({:.1}%)",
            s14_usados.len(),
            total_s14,
            final_coverage_percent
        ));
    }

    let elapsed = start.elapsed();
    let cobertura_percentual = (s14_usados.len() as f64 / total_s14 as f64) * 100.0;

    println!(
        "Algoritmo concluído com {} S15 em {:.2?}",
        sb15_14.len(),
        elapsed
    );
    println!(
        "Cobertura: {}/{} S14 ({:.2}%)",
        s14_usados.len(),
        total_s14,
        cobertura_percentual
    );

    let main_out = File::create("output/SB15_14.csv").expect("Falha ao criar SB15_14.csv");
    let mut main_writer = BufWriter::new(main_out);
    for &mask in &sb15_14 {
        let seq = mask_para_seq(mask);
        let line = seq
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        writeln!(main_writer, "{}", line).expect("Erro escrevendo solução");
    }
    println!("Cópia também salva em 'output/SB15_14.csv'");
}
