use rand::*;
// use std::{thread, time::Duration};
use std::iter::Iterator;

const N: usize = 200_000_000;


// Сумма всех чисел
fn sum_up(it: impl Iterator<Item=u64>) -> u64 {
    it.sum()
}

fn sum_up2(it: impl Iterator<Item=u64>) -> u64 {
    it.fold(0, |acc, x| acc + x)
}

// Максимальное число среди тех, что делятся на 9
fn my_max(it: impl Iterator<Item=u64>) -> Option<u64> {
    it.filter(|&x| x % 9 == 0).max() // _by_key(|&x| ((x as f64).sin() * 1000000.0) as i64)
}

fn my_max2(it: impl Iterator<Item=u64>) -> Option<u64> {
    it.filter(|&x| x % 9 == 0).fold(None, |acc, x| {
        match acc {
            None => Some(x),
            Some(y) => Some(std::cmp::max(x, y))
        }
    })
}

// Отфильтровать итератор
fn filter9(it: impl Iterator<Item=u64>) -> impl Iterator<Item=u64> {
    it.filter(|&x| x % 9 == 0)
}



fn gen_random(n: usize) -> impl Iterator<Item=u64> {
    let mut rng = thread_rng();
    (0..n).map(move|_i|rng.gen_range(0..=1000))
}

fn main() {
    // Генерация
    let randoms = gen_random(N);

    // Подсчёт
    let m = my_max(randoms);

    println!("{:?}", m);

    // thread::sleep(Duration::from_millis(30000));

}
