use std::collections::btree_set::Range;
use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::io::{self, BufRead, stdin};
use std::ops::Bound::Included;
use std::path::Path;

fn get_all_by_prefix(dict: &BTreeSet<String>, prefix: &str) -> Vec<String> {
    let next = format!("{}я", prefix);
    let range: Range<String> = dict.range((Included(prefix.to_string()), Included(next)));

    range.into_iter().cloned().collect()
}

fn is_winner(dict: &BTreeSet<String>, prefix: &str) -> Option<String> {
    let layer = get_all_by_prefix(dict, prefix);
    if layer.is_empty() {
        return Some(format!("!{} -- No such word!", prefix));
    }
    if layer.iter().any(|s| s.as_str() == prefix) {
        return Some(format!("{} -- in dictionary!", prefix));
    }

    let pos = prefix.len();
    let next_letter: HashSet<char> = layer.iter().map(|s| {
        s.chars().nth(pos).unwrap()
    }).collect();
    for c in next_letter {
        let my_move = format!("{}{}", prefix, c);
        if is_winner(dict, &my_move).is_none() {
            return Some(my_move);
        }
    }
    None
}

fn main() {
    let path = Path::new("../wordlist.10000.txt");
    let file = File::open(path).unwrap_or_else(|err| {
        panic!("{:?}", err);
    });
    let (v, _errors): (Vec<_>, Vec<_>) = io::BufReader::new(file).lines().partition(|x| x.is_ok());
    println!("{}", v.len());

    let v: Vec<String> = v.into_iter().filter_map(|x| x.ok()).collect();
    let m: BTreeSet<String> = v.into_iter().filter(|s|s.len() > 1).collect();

    let mut current: String = "".to_string();
    loop {
        let mut input = "".to_string();
        stdin().lock().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input == "?" {
            println!("{:?}", get_all_by_prefix(&m, &current));
        } else if input.len() != current.len() + 1 || input[0..current.len()] != current {
            println!("Cheater!!");
        } else if let Some(m) = is_winner(&m, &input) {
            println!(">{}", m);
            if &m[0..1] != "!" {
                current = m;
            }
        } else {
            println!("Surrender (( Start next word");
            current = "".to_string();
        }
    }
}
