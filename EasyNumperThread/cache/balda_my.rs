use dashmap::DashSet;
use parking_lot::RwLock;
use std::collections::btree_set::Range;
use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::io::{self, stdin, BufRead};
use std::path::Path;
use std::thread::spawn;
use std::time::Duration;

fn get_all_by_prefix(dict: &DashSet<String>, prefix: &str) -> Vec<String> {
    // WHERE S like 'prefix%'
    // WHERE S >= 'prefix' ans S < 'prefixяяяяяяяяяяя'
    //
    // let next = format!("{}я", prefix);
    // let range: Range<String> = dict.range(prefix.to_string()..=next);
    // (0..10)
    // (Included(0), Excluded(10))

    // range.into_iter().cloned().collect()
    vec![]
}     

#[derive(PartialEq)]
enum PositionEstimation {
    BadPosition,
    AlreadyWin,
    AlreadyLost(String),
    // (какой_ход_делаю, потенциальная_длина_партии)
    CannotWin(String, usize),
    // (какой_ход_делаю, потенциальная_длина_партии)
    CanWin(String, usize),
}

fn is_winner(dict: &DashSet<String>, prefix: &str) -> PositionEstimation {
    let layer = get_all_by_prefix(dict, prefix);
    if layer.is_empty() {
        return PositionEstimation::BadPosition;
    }
    if layer.iter().any(|s| s.as_str() == prefix) {
        return PositionEstimation::AlreadyWin;
    }
    if *FOUND.read() {
        return PositionEstimation::AlreadyWin;
    }

    let pos = prefix.len();
    let next_letter: HashSet<char> = layer.iter().map(|s| s.chars().nth(pos).unwrap()).collect();

    let mut longest_lose = ("".to_string(), 0);
    for c in next_letter {
        let my_move = format!("{}{}", prefix, c);
        match is_winner(dict, &my_move) {
            PositionEstimation::BadPosition => panic!(),
            PositionEstimation::AlreadyWin => {}
            PositionEstimation::CannotWin(_, n) => {
                return PositionEstimation::CanWin(my_move, n + 1);
            }
            PositionEstimation::AlreadyLost(_) => {
                *FOUND.write() = true;
                return PositionEstimation::CanWin(my_move, 1);
            }
            PositionEstimation::CanWin(_, n) => {
                if n + 1 > longest_lose.1 {
                    longest_lose = (my_move.to_string(), n + 1);
                }
            }
        }
    }
    // let longest = layer.iter().max_by_key(|&s| s.len()).unwrap().clone();
    // let m = longest[..pos + 1].to_string();
    if layer.contains(&longest_lose.0) {
        PositionEstimation::AlreadyLost(longest_lose.0)
    } else {
        PositionEstimation::CannotWin(longest_lose.0, longest_lose.1)
    }
}

fn is_winner_first_move(prefix: &str) -> PositionEstimation {
    const THREAD_COUNT: usize = 4;
    let next_move: Vec<String> = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| format!("{}{}", prefix, c))
        .collect();

    let parts: Vec<Vec<String>> = next_move
        .chunks(next_move.len() / THREAD_COUNT + 1)
        .map(|x| x.to_vec())
        .collect();

    let handles: Vec<_> = parts
        .into_iter()
        .map(|part| {
            // let dict = dict.clone();
            spawn(move || -> bool {
                for s in &part {
                    if let PositionEstimation::CannotWin(_, _) = is_winner(&DICT, s) {
                        return true;
                    }
                }
                false
            })
        })
        .collect();

    let mut victory = false;
    for handle in handles {
        if handle.join().unwrap() {
            victory = true;
        }
    }

    if victory {
        PositionEstimation::AlreadyWin
    } else {
        PositionEstimation::CannotWin("((".to_string(), 0)
    }
}

use lazy_static::lazy_static;
lazy_static! {
    static ref DICT: DashSet<String> = DashSet::new();
    static ref FOUND: RwLock<bool> = RwLock::new(false);
}

fn main() {
    let path = Path::new("../wordlist.10000.txt");
    let file = File::open(path).unwrap_or_else(|err| {
        panic!("{:?}", err);
    });
    let (v, _errors): (Vec<_>, Vec<_>) = io::BufReader::new(file).lines().partition(|x| x.is_ok());
    println!("{}", v.len());

    let v: Vec<String> = v.into_iter().filter_map(|x| x.ok()).collect();
    {
        for s in v.into_iter().filter(|s| s.len() > 1) {
            DICT.insert(s);
        }
    }
    // let m: BTreeSet<String> = v.into_iter().filter(|s| s.len() > 1).collect();

    let mut current: String = "".to_string();
    loop {
        let mut input = "".to_string();
        stdin().lock().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input == "?" {
            println!("{:?}", get_all_by_prefix(&DICT, &current));
        } else if input.len() != current.len() + 1 || input[0..current.len()] != current {
            println!("Cheater!!");
        } else {
            *FOUND.write() = false;
            match is_winner_first_move(&input) {
                PositionEstimation::CanWin(m, _) | PositionEstimation::CannotWin(m, _) => {
                    println!(">{}", m);
                    current = m;
                }
                PositionEstimation::AlreadyWin | PositionEstimation::BadPosition => {
                    println!("I won! Start next word");
                    current = "".to_string();
                }
                PositionEstimation::AlreadyLost(m) => {
                    println!(">{}", m);
                    println!("I lose (( Start next word");
                    current = "".to_string();
                }
            }
        }
    }
}
