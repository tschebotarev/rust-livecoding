// https://leetcode.com/problems/longest-increasing-path-in-a-matrix/
// ДЗ:
// https://leetcode.com/problems/maximal-rectangle/

use rand::Rng;

fn main() {
    println!("answer: {}",operate(generator(3)));
}

fn generator(n: i32) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();
    let mut a:Vec<Vec<i32>> = vec![];
    for _y in 0..n {
        let mut b:Vec<i32> = vec![];
        for _x in 0..n {
            b.push(rng.gen_range(0..10));
        }
        a.push(b);
    }
    a
}

fn operate(matrix: Vec<Vec<i32>>) -> i32 {
    for i in 0..matrix.len() { println!("{:?}",matrix[i]); }

    let mut array:Vec<Vec<i32>> = vec![vec![0;matrix[0].len()];matrix.len()];

    let mut max = 0;

    for y in 0..matrix.len() {
        let t = matrix[y].len();
        for x in 0..t {
            let c = test(&matrix,&mut array,(x,y),-1);
            if max<c {
                max = c;
            }
        }
    }

    println!("==============");
    for i in 0..array.len() { println!("{:?}",array[i]); }


    max
}

fn test(matrix: &Vec<Vec<i32>>, array:&mut Vec<Vec<i32>>, coo: (usize,usize), old_num: i32) -> i32 { // mut road: Vec<(usize,usize)>, , old_go_flag: bool
    if old_num>=matrix[coo.1][coo.0] {
        //println!("!!!");
        return 0;
    }
    if array[coo.1][coo.0]!=0 {
        //print
        return array[coo.1][coo.0];
    }

    //if old_go_flag { road.push(old_go); }
    
    let mut a = 0;
    let mut b = 0;
    if coo.0>0                 { a = test(matrix,array,(coo.0-1,coo.1),matrix[coo.1][coo.0]) } if a>b { b = a; } // left
    if coo.0<matrix[0].len()-1 { a = test(matrix,array,(coo.0+1,coo.1),matrix[coo.1][coo.0]) } if a>b { b = a; } // right
    if coo.1>0                 { a = test(matrix,array,(coo.0,coo.1-1),matrix[coo.1][coo.0]) } if a>b { b = a; } // up
    if coo.1<matrix.len()-1    { a = test(matrix,array,(coo.0,coo.1+1),matrix[coo.1][coo.0]) } if a>b { b = a; } // down

    array[coo.1][coo.0] = b+1;
    b+1
}