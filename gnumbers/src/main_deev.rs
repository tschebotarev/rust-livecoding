use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

use std::collections::HashMap;

mod generator;

fn main() -> Result<(), Error>{
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args.len() {
        1 => {
                let mut map = HashMap::new();
                let input = File::open("input.txt")?;
                let buffered = BufReader::new(input);

                for line in buffered.lines() {
                    if let Ok(num) = line?.parse::<u64>() {
                        //println!("parsed {}", num);
                        let count = map.entry(num).or_insert(0);
                        *count +=1;                  
                    };
                }
                let mut vec: Vec<(&u64, &u32)> = map.iter().collect();
                vec.sort_by(|a, b| b.1.cmp(a.1));
                //println!("{:?}", vec);
                for (key, value) in &vec {
                    println!("{},{}", key, value)
                }   
        },
        2 => {
            if let Ok(n) = args[1].parse() {
                generator::generate(n)
            }
        }
        _ => {}
    };
    Ok(())
}