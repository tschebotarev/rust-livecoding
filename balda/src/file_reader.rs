use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

pub fn read_file(s:String) -> Result<Vec<String>, Error>{
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    let mut out:Vec<String> = vec![];
    match args.len() {
        1 => {
            let input = File::open(s)?;
            let buffered = BufReader::new(input);

            for line in buffered.lines() {
                //let a = line.unwrap();
                if let Ok(a) = line {
                    //println!("{:?}",a);
                    out.push(a);
                }
                
            }
        },
        _ => {}
    };
    Ok(out)
}


