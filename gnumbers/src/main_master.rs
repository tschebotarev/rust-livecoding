use std::array::IntoIter;
use std::env;
use std::io::{self, BufRead, BufReader, Read};

mod generator;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args.len() {
        1 => {
            solution();
        },
        2 => {
            if let Ok(n) = args[1].parse() {
                generator::generate(n)
            }
        }
        _ => {}
    }
}


fn process<I>(it: I)
    where I: Iterator<Item = u64>,
{
    for _x in it {
        
    }

}


struct StdinWrapper {
    s: io::Stdin
}

impl<'a> IntoIterator for StdinWrapper {
    type Item = u64;
    type IntoIter = std::iter::Map<std::iter::Flatten<std::io::Lines<std::io::StdinLock<'a>>>, _>;

    fn into_iter(self) -> Self::IntoIter {
        self.s.lock().lines().flatten().map(|s| s.parse::<u64>().unwrap_or_default())
    } 
}

fn from_stdin() {
    let stream = io::stdin();
    stream.lock().lines().flatten().map(|s| s.parse::<u64>().unwrap_or_default())
}


fn solution() {
    process( from_stdin() );

    let mut m = BTreeMap::<char, u32>::new();
    let mut m = HashMap::<char, u32>::new();

    *m.entry('a').or_default() += 1;
}

use std::collections::{BTreeMap, HashMap};

use proc_macro::token_stream::IntoIter;