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

// Максимальное число среди тех, что делятся на 9
fn my_max(it: impl Iterator<Item=u64>) -> u64 {
    0
}

fn gen_random(n: usize) -> impl Iterator<Item=u64> {
    let mut rng = thread_rng();
    (0..N).map(move|_i|rng.gen_range(0..=1000))
}

fn main() {
    let mut rng = thread_rng();
    // Генерация
    // let mut v = Vec::<u64>::new();

    let randoms = (0..N).map(|_i|rng.gen_range(0..=1000));

    // Подсчёт
    let m = my_max(randoms);


    println!("{}", m);

    // thread::sleep(Duration::from_millis(30000));
    // println!("{}", v[N - 1]);

}
