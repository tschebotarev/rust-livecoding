use rand::*;
use std::{thread, time::Duration};


const N: usize = 200_000_000;


fn sum_up(v: &Vec<u64>) -> u64 {
    let mut s = 0;
    for i in 0..N {
        s += v[i]
    }
    thread::sleep(Duration::from_millis(30000));
    s
}

fn main() {
    let mut rng = thread_rng();
    // Генерация
    let mut v = Vec::<u64>::new();
    for _i in 0..N {
        v.push(rng.gen_range(0..=1000));
    }
    // println!("{:?}", v);

    // Подсчёт
    let s = sum_up(&v);

    println!("{}", s);

    // thread::sleep(Duration::from_millis(30000));
    println!("{}", v[N - 1]);

}
