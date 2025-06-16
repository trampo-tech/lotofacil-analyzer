// ================================
// src/exercicio4.rs
// CENÁRIO C3: Cobrir todas S12 usando SB15_12
// ================================
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::{create_dir_all, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;

#[inline] fn seq_para_mask(seq:&[u8])->u32{ let mut m=0; for &n in seq{m|=1<<(n-1);} m }
#[inline] fn mask_para_seq(mask:u32)->Vec<u8>{ (0..25).filter_map(|i| if mask&(1<<i)!=0{Some((i+1) as u8)}else{None}).collect() }
fn carregar_s12(path:&str)->HashSet<u32>{ let f=File::open(path).unwrap(); let mut s=HashSet::with_capacity(5_200_300); for l in BufReader::new(f).lines(){ let row=l.unwrap(); let nums=row.split(',').map(|s|s.parse().unwrap()).collect::<Vec<u8>>(); s.insert(seq_para_mask(&nums)); } s }

pub fn executar() {
    create_dir_all("output").ok(); println!("Carregando S12...");
    let mut uncovered=carregar_s12("output/saida_S12.csv"); let total=uncovered.len(); println!("S12: {} combos a cobrir", total);
    let mut solution=Vec::with_capacity(total/455+1); let start=Instant::now();
    let remove3=(0..15).combinations(3).collect::<Vec<_>>();
    for combo in (1u8..=25).combinations(15) {
        let m15=seq_para_mask(&combo);
        let mut covered=false;
        for rem in &remove3 {
            let mut sub=m15;
            for &i in rem { sub &= !(1<<(combo[i]-1)); }
            if uncovered.remove(&sub) { covered=true; }
        }
        if covered { solution.push(m15); if uncovered.is_empty(){break;} }
    }
    println!("Cobriu S12 com {} S15 em {:?}", solution.len(), start.elapsed());
    let mut w=BufWriter::new(File::create("output/SB15_12.csv").unwrap());
    for &m in &solution { let seq=mask_para_seq(m); writeln!(w, "{}", seq.iter().map(|n|n.to_string()).collect::<Vec<_>>().join(",")).unwrap(); }
    println!("SB15_12 salvo");
}