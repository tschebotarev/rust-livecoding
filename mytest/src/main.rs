use std::mem::size_of;

// radix-sort

/*
https://docs.rs/num/0.4.0/num/trait.Integer.html
https://docs.rs/num/0.4.0/num/trait.Num.html
https://leetcode.com/problems/zuma-game/
https://leetcode.com/problems/interleaving-string/
https://leetcode.com/problems/maximal-rectangle/
*/

fn ultra_sort(array:&mut Vec<i32>) { 
    let steps = size_of::<i32>()*2;
    //println!("{}",steps);
    for a in 0..steps {
        let array_save = array.clone();
        //let mut test:Vec<u32> = vec![0;10];
        let mut test:Vec<u32> = vec![0;16];
        for i in 0..array.len() {
            //let number = ((array_save[i]%(10_u32.pow(a)))/(10_u32.pow(a-1))) as usize;
            let mut number = ((array_save[i]>>(4*a))&15) as usize;
            if a==steps-1 {
                number ^= 8;
            }
            //println!("{} {} {}",i,array_save[i],number);
            test[number] += 1;
            array[i] = 0;
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
            //let number = ((array_save[i]%(10_u32.pow(a)))/(10_u32.pow(a-1))) as usize;
            let mut number = ((array_save[i]>>(4*a))&15) as usize;
            if a==steps-1 {
                number ^= 8;
            }
            //println!("{} {} {}",i,array_save[i],number);
            array[place[number] as usize] = array_save[i];
            place[number] += 1;
        }
    }
}

fn main() {
    let mut array = vec![12,23,24,17,-36,56,32,11];
    println!("basic:  {:?}", array);
    ultra_sort(&mut array);
    println!("answer: {:?}", array);
}


