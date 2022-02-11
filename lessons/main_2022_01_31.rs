use num::Integer; // num = "0.2.1"
use std::mem::size_of;

// radix-sort

/*
https://docs.rs/num/0.4.0/num/trait.Integer.html
https://docs.rs/num/0.4.0/num/trait.Num.html
https://leetcode.com/problems/zuma-game/
https://leetcode.com/problems/interleaving-string/
https://leetcode.com/problems/maximal-rectangle/
*/

fn fn_for_sort<T>(n:T, a:usize, steps:usize) -> usize 
where T: num::Integer,
{
    // ((array_save[i]>>(4*a))&15) as usize; // start
    let number = ((n >> ((4*a))&15) ) as usize;
    
    // для отрицательных чисел
    if a==steps-1 {
        // number ^= 8;
        return number^8;
    }
    number
}

fn ultra_sort<T>(array:&mut Vec<T>) 
where T: num::Integer + Clone, //  + Default
{ 
    let steps = size_of::<T>()*2;
    //println!("{}",steps);
    for a in 0..steps {
        let array_save = array.clone();
        //let mut test:Vec<u32> = vec![0;10];
        let mut test:Vec<u32> = vec![0;16];
        for i in 0..array.len() {
            let mut number = fn_for_sort(array_save[i],a,steps);
            //println!("{} {} {}",i,array_save[i],number);
            test[number] += 1;
            array[i] = 0 as T;
        }
        //println!("{:?}",test);
        //let mut place:Vec<u32> = vec![0;10];
        let mut place:Vec<u32> = vec![0;16];
        place[0] = 0;
        for i in 1..16 {
            place[i] = place[i-1] + test[i-1];
        }
        //println!("{:?}",place);
        for i in 0..array.len() {
            let mut number = fn_for_sort(array_save[i],a,steps);
            //println!("{} {} {}",i,array_save[i],number);
            array[place[number] as usize] = array_save[i];
            place[number] += 1;
        }
    }
}

fn main() {
    let mut array:Vec<i32> = vec![12,23,24,17,-36,56,32,11];
    println!("basic:  {:?}", array);
    ultra_sort(&mut array);
    println!("answer: {:?}", array);
}


