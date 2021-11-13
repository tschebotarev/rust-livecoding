use std::io::{self, BufWriter, Write};
use rand::*;

pub fn generate(n: usize) {
    let mut rng = thread_rng();
    const MIN_VAL: u64 = 10_000_000_000;
    const MAX_VAL: u64 = 99_999_999_999;

    let mut stream = BufWriter::new(io::stdout());
    let mut species = Vec::<u64>::with_capacity((2.1 * n as f64).sqrt() as usize);

    for _i in 0..n {
        let j = rng.gen_range(0..=species.len());
        let num = if j < species.len() {
            species[j]
        } else {
            let num = rng.gen_range(MIN_VAL..=MAX_VAL);
            species.push(num);
            num
        };
        if let Err(e) = writeln!(stream, "{}", num) {
            eprintln!("{}", e);
            return;
        }
    }
}