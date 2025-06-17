use std::time::Instant;

use crate::ex2;
use crate::ex3;
use crate::ex4;
use crate::ex5;

pub fn executar() {
    println!("Iniciando análise de tempo dos Programas 2 a 5\n");

    let start_ex2 = Instant::now();
    ex2::executar(None);
    println!("→ PROGRAMA 2 rodou em {:.2?}\n", start_ex2.elapsed());

    let start_ex3 = Instant::now();
    ex3::executar(None);
    println!("→ PROGRAMA 3 rodou em {:.2?}\n", start_ex3.elapsed());

    let start_ex4 = Instant::now();
    ex4::executar(None);
    println!("→ PROGRAMA 4 rodou em {:.2?}\n", start_ex4.elapsed());

    let start_ex5 = Instant::now();
    ex5::executar(None);
    println!("→ PROGRAMA 5 rodou em {:.2?}\n", start_ex5.elapsed());

    println!("Análise completa!");
}
