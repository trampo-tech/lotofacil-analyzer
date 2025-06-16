use std::time::Instant;

use crate::ex2;
use crate::ex3;
use crate::ex4;
use crate::ex5;

pub fn executar() {
    println!("Iniciando análise de tempo dos Programas 2 a 5\n");

    let start = Instant::now();
    ex2::executar();
    println!("→ PROGRAMA 2 rodou em {:.2?}\n", start.elapsed());

    let start = Instant::now();
    ex3::executar();
    println!("→ PROGRAMA 3 rodou em {:.2?}\n", start.elapsed());

    let start = Instant::now();
    ex4::executar();
    println!("→ PROGRAMA 4 rodou em {:.2?}\n", start.elapsed());

    let start = Instant::now();
    ex5::executar();
    println!("→ PROGRAMA 5 rodou em {:.2?}\n", start.elapsed());

    println!("Análise completa!");
}
