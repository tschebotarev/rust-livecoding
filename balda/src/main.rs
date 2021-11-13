use std::io::{BufRead, stdin};

mod file_reader;
use file_reader::read_file;

use rand::Rng;

fn test_people(array: &Vec<String>, start_word: String) -> i32 {
    for a in array {
        if a==&start_word { 
            return 1;
        }
    }
    let size = start_word.to_string().len();
    for a in array {
        if a.len()>=size {
            if a[0..size]==start_word {
                return 0;
            }
        }
    }
    return 2;
}

fn find_word(array: &Vec<String>, start_word: String) -> Vec<String> {
    let mut answer: Vec<String> = Vec::new();
    let size = start_word.to_string().len();
    for a in array {
        if a.len()>=size {
            if a[0..size]==start_word {
                answer.push(a.clone());
            }
        }
    }
    answer
}

fn generator_go(array: &Vec<String>, start_word: String) -> Vec<String> {
    let alphabet:Vec<char> = "abcdefghijklmnopqrstuwvxyz".chars().collect();
    let i_can_go:Vec<String> = find_word(&array,start_word.to_string());
    let mut answer:Vec<String> = Vec::new();

    if i_can_go.len()>0 { 
        for i in 0..26 {
            let new = format!("{}{}",start_word,alphabet[i].to_string());
            let mut flag_no_end:bool = true;
            let mut flag_exists:bool = false;
            let size = start_word.to_string().len()+1;
            for a in &i_can_go {
                if a==&new { 
                    flag_no_end = false;
                }
                if a.len()>=size {
                    if a[0..size]==new { 
                        flag_exists = true;
                    }
                }
            }
            if flag_no_end && flag_exists {
                answer.push(new); // alphabet[i]
            }
        }
    }
    answer
}

fn best_go(array: &Vec<String>, start_word: String, i_am_master: bool) -> (String,String) {
    let can_move:Vec<String> = generator_go(array, start_word.clone());
    //if start_word.len()==2 { println!("{:?}",can_move); }
    
    // not random
    let mut best_word:Vec<String> = Vec::new();
    let mut long_lose:String = start_word.clone();
    for a in &can_move {
        let a = best_go(array, a.clone(), false);
        if a.0=="robot lost".to_string() { 
            best_word.push(a.0.clone()); 
        }
        else if a.1.len()>long_lose.len() { long_lose = a.1.clone(); }
    }

    // random...
    let mut rng = rand::thread_rng();
    //let non = "".to_string();
    if best_word.len()==0 { 
        if i_am_master { return ("robot lost".to_string(),long_lose); }
        else { return ("robot lost".to_string(),long_lose); }
    } 
    else { return (best_word[rng.gen_range(0..best_word.len())].to_string(),long_lose); }
}

fn main() {

    let path = "D:/GitHub/rust-livecoding/balda/src/engmix.txt"; 

    //read file
    let input_array_from_file = read_file(path.to_string()).unwrap();
    println!("quantity words: {}",input_array_from_file.len());
    //let start_world = "ma";
    //println!("answer: {:?}",best_go(&input_array_from_file,start_world.to_string()));

    loop {
        let mut game_line = "".to_string();
        println!("people: ");
        stdin().lock().read_line(&mut game_line).unwrap();
        game_line = game_line.trim().to_string();
        //println!("{:?}",game_line);
        let a = test_people(&input_array_from_file, game_line.clone());
        if a==0 {
            let t = best_go(&input_array_from_file,game_line.to_string(), true).0.to_string();
            println!("robot: {}",t);
        }
        else if a==1 { println!("you lost"); }
        else { println!("there is no such word in the dictionary"); }
    }
}
