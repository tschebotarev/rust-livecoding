use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn are_connected(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let n_diff = s1.chars().zip(s2.chars()).filter(|(x, y)| x != y).count();
    n_diff == 1
}

// Количество слов, которые на расстоянии не более 3 в графе
//

// СТОП
// СТОЛ

fn main() {
    let path = Path::new("../engmix.txt");
    let file = File::open(path).unwrap_or_else(|err| {
        panic!("{:?}", err);
    });
    let (v, _errors): (Vec<_>, Vec<_>) = io::BufReader::new(file).lines().partition(|x| x.is_ok());
    println!("{}", v.len());

    for s in v {
        let s = s.unwrap();
    }
}

#[test]
fn test_connected() {
    assert!(are_connected("мама", "сама"))
}

#[test]
fn test_not_connected() {
    assert!(!are_connected("мама", "папа"))
}

#[test]
fn test_identical() {
    assert!(!are_connected("мама", "мама"))
}
