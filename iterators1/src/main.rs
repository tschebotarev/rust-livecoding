use rand::*;
// use std::{thread, time::Duration};


const N: usize = 20;
fn main() {
    let mut rng = thread_rng();
    let mut v = Vec::<u64>::new();
    for _i in 0..N {
        v.push(rng.gen_range(0..=1000));
    }
    println!("{:?}", v);

    let mut s = 0;
    for i in 0..N {
        s += v[i]
    }
    println!("{}", s);

    // thread::sleep(Duration::from_millis(60000));
    // println!("{}", v[N - 1]);

}
