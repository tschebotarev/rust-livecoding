use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng(); 
    let a = rng.gen_range(0..2);
    let b = rng.gen_range(0..2);

    //let mut test:i32 = 0;
    let test:i32;
    
    if a==b {
        test = 100;
    }
    else {
        test = 200;
    }
    
    println!("{}",test);
}