use std::iter::Iterator;

pub fn my_max(it: impl Iterator<Item=u64>) -> Option<u64> {
    let mut s = 0;
    for x in it {
        if x%9==0&&x>s { s=x; }
    }
    if s==0 { return Option::None; }
    Option::Some(s)
}

pub fn sum_up(it: impl Iterator<Item=u64>) -> u64 {
    let mut s = 0;
    for x in it {
        s += x;
    }

    s
}

/*fn gen_random(n: usize) -> impl Iterator<Item=u64> {
    let mut rng = thread_rng();
    let res = (0..N).map(move|_i|rng.gen_range(0..=1000));
    //let tmp = rng.gen_range(0..=1000);
    res
}*/