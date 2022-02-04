use rand::*;
//use std::{thread, time::Duration};
use std::iter::Iterator;

mod test;
use test::my_max;

mod file;
use file::read_file;

const N: usize = 200_000; // _000

fn main() {

    let a = read_file();
    for i in a { 
        //println!("{:?}",i); 
        let s = 0;//my_max(i.iter());
        let s = my_max(i.iter());

        println!("{:?}", s);
    }


    /*let mut rng = thread_rng();
    let randoms = (0..N).map(|_i|rng.gen_range(0..=1000));
    
    let s = my_max(randoms);
    println!("{:?}", s);*/

    // thread::sleep(Duration::from_millis(30000));
    // println!("{}", v[N - 1]);
    
    
}
