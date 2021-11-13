// use std::env;
// use std::fs;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

use std::collections::HashMap;

mod generator;

//mod generator;
//pub use generator::generate;

//use std::collections::BTreeMap;

use std::time::{SystemTime, UNIX_EPOCH}; // for time testing
fn millis() -> f64 { // https://qastack.ru/programming/26593387/how-can-i-get-the-current-time-in-milliseconds
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64)/(1000.0)
}



fn read_file() -> Result<(Vec<u64>), Error>{
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    let mut out:Vec<u64> = vec![];
    match args.len() {
        1 => {
                let mut map = HashMap::new();
                let input = File::open("C:/Users/Ni3na/Documents/GitHub/rust-livecoding/gnumbers/src/input2.txt")?;
                let buffered = BufReader::new(input);

                for line in buffered.lines() {
                    //println!();
                    if let Ok(num) = line?.parse::<u64>() {
                        //println!("parsed {}", num);
                        out.push(num);
                        let count = map.entry(num).or_insert(0);
                        *count +=1;                  
                    };
                }
                /*//println!("--------");
                let mut vec: Vec<(&u64, &u32)> = map.iter().collect();
                vec.sort_by(|a, b| b.1.cmp(a.1));
                //println!("{:?}", vec);
                for (key, value) in &vec {
                    println!("{},{}", key, value)
                }  */ 
        },
        2 => {
            if let Ok(n) = args[1].parse() {
                generator::generate(n)
            }
        }
        _ => {}
    };
    Ok((out))
}

fn main() {
    let a = read_file();
    //let array = a:Ok();
    //for i in array.0 { println("{}",i); }
}

/*fn main() {
    //generate(10);
    let mut input:Vec<u64> = vec![78471671870, 78471671870, 78471671870, 75002370706, 75002370706, 78471671870, 75002370706, 78471671870, 75002370706, 69890740478];

    let time_start = millis();
    
    input.sort();
    let mut count:Vec<u64> = vec![1];
    let mut a = count.len()-1;
    for i in 1..input.len() {
        if input[i]==input[i-1] { count[a]+=1; }
        else { a+=1; count.push(1); }
    }
    for i in 0..count.len() {
        for _u in 1..count[i] {
            input.remove(i);
        }
    }
    let mut count_2 = count.to_vec();
    count_2.sort();

    println!("A) quantity differents numbers: {}", input.len());
    println!("B) array:");
    for i in (0..count_2.len()).rev() { 
        let b = count_2.iter().position(|&r| r == count_2[i]).unwrap();
        println!("      {} {}", input[b], count[b]);
    }
    let time_end = millis();
    println!("C) work time: {}",time_end-time_start);
}*/
