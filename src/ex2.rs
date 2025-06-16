use itertools::Itertools;
use std::collections::HashSet;
use std::fs::{create_dir_all, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;

#[inline]
fn seq_para_mask(seq: &[u8]) -> u32 {
    let mut mask = 0;
    for &n in seq {
        mask |= 1 << (n - 1);
    }
    mask
}

#[inline]
fn mask_para_seq(mask: u32) -> Vec<u8> {
    (0..25)
        .filter_map(|i| if (mask & (1 << i)) != 0 { Some((i + 1) as u8) } else { None })
        .collect()
}

fn carregar_s14(path: &str) -> HashSet<u32> {
    let file = File::open(path).expect("Falha ao abrir S14 CSV");
    let reader = BufReader::new(file);
    let mut set = HashSet::with_capacity(4_500_000);
    for line in reader.lines() {
        let l = line.expect("Erro lendo linha");
        let nums = l.split(',').map(|s| s.parse::<u8>().unwrap()).collect::<Vec<_>>();
        set.insert(seq_para_mask(&nums));
    }
    set
}

pub fn executar() {
    create_dir_all("output").expect("Não pôde criar output");

    println!("Carregando S14...");
    let mut uncovered = carregar_s14("output/saida_S14.csv");
    let total_s14 = uncovered.len();
    println!("S14 carregado: {} combinações", total_s14);

    let mut solution = Vec::with_capacity(total_s14 / 15 + 1);
    let start = Instant::now();

    for combo in (1u8..=25).combinations(15) {
        let mask15 = seq_para_mask(&combo);
        let mut covered = 0;
        for &n in &combo {
            let sub = mask15 & !(1 << (n - 1));
            if uncovered.remove(&sub) {
                covered += 1;
            }
        }
        if covered > 0 {
            solution.push(mask15);
            if uncovered.is_empty() {
                break;
            }
        }
    }

    let elapsed = start.elapsed();
    println!("Cobertura completa com {} S15 em {:.2?}", solution.len(), elapsed);

    let out = File::create("output/SB15_14.csv").expect("Falha ao criar SB15_14.csv");
    let mut writer = BufWriter::new(out);
    for &mask in &solution {
        let seq = mask_para_seq(mask);
        let line = seq.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        writeln!(writer, "{}", line).expect("Erro escrevendo solução");
    }

    println!("SB15_14 salvo em 'output/SB15_14.csv'");
}
