mod file_reader;
use file_reader::read_file;

use std::collections::HashMap;

fn are_connected(s1: &str, s2: &str) -> bool { // test world
    if s1.len() != s2.len() { return false; }
    let n_diff = s1.chars().zip(s2.chars()).filter(|(x,y)| x!=y).count();
    n_diff==1
}

fn output(my_hashmap:HashMap<String, Vec<String>>, world:&str, probel:i32, count:i32) {
    for _i in 0..probel { print!("   "); }
    println!("{}",world);
    if count>0 {
        for new_word in my_hashmap[world].iter() {
            output(my_hashmap.clone(), new_word, probel+1, count-1);
        }
    }
}

fn main() {
    // start const
    let find_world = "make";
    let path = "D:/GitHub/rust-livecoding/fileReadHashMapFind/src/engmix.txt";

    //read file
    let input_array_from_file = read_file(path.to_string()).unwrap();
    println!("quantity words: {}",input_array_from_file.len());

    // write in HashMap
    let mut my_hashmap:HashMap<String, Vec<String>> = HashMap::new();
    for start_world in input_array_from_file.iter() { // анализ всех слов
        let mut cache_for_hashmap:Vec<String> = vec![];       // создадим кеш
        for i in input_array_from_file.iter() {        // пробежимся по всем словам 
            if are_connected(start_world, &i) {         // слово подхоит по нашим правилам
                if cache_for_hashmap.contains(&i)==false {     // чтобы дубликаты не заносились
                    cache_for_hashmap.push(i.clone());         // заносим в "кеш"
                }
            }
        }
        my_hashmap.insert(start_world.clone(), cache_for_hashmap);
    }

    // output
    output(my_hashmap.clone(), find_world, 0, 2);
    
}
