use rand::*;
use std::{thread, time::Duration};
use std::iter::Iterator;

const N: usize = 200_000_000;


// 
fn sum_up(it: impl Iterator<Item=u64>) -> u64 {
    let mut s = 0;
    for x in it {
        s += x;
    }

    s
}

fn gen_random(n: usize) -> impl Iterator<Item=u64> {
    let mut rng = thread_rng();
    let res = (0..N).map(move|_i|rng.gen_range(0..=1000));
    let tmp = rng.gen_range(0..=1000);
    res
}

fn main() {
    let mut rng = thread_rng();
    // Генерация
    // let mut v = Vec::<u64>::new();

    let randoms = (0..N).map(|_i|rng.gen_range(0..=1000));

    // Подсчёт
    let s = sum_up(randoms);

    println!("{}", s);

    // thread::sleep(Duration::from_millis(30000));
    // println!("{}", v[N - 1]);

}
