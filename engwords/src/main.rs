use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;


fn are_connected(s1: &str, s2: &str) -> bool {
    true
}

// Количество слов, которые на расстоянии не более 3 в графе
// 

// СТОП
// СТОЛ




fn main() {
    let path = Path::new("../engmix.txt");
    let file = File::open(path).unwrap_or_else(|err| {panic!("{:?}", err);});
    let (v, _errors): (Vec<_>, Vec<_>) = io::BufReader::new(file).lines().
        partition(|x|x.is_ok());
    println!("{}", v.len());

    for s in v {
        let s = s.unwrap();

        
        
    }
    
}
