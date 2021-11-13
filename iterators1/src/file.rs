use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

pub fn read_file() -> Result<(Vec<u64>), Error>{
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    let mut out:Vec<u64> = vec![];
    match args.len() {
        1 => {
                let input = File::open("D:/GitHub/rust-livecoding/iterators1/src/input2.txt")?;
                let buffered = BufReader::new(input);

                for line in buffered.lines() {
                    //println!();
                    if let Ok(num) = line?.parse::<u64>() {
                        //println!("parsed {}", num);
                        out.push(num);              
                    };
                }
        },
        _ => {}
    };
    Ok((out))
}


