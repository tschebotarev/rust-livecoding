use itertools::Itertools;
use rand::*;

type Aux = usize;

pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    let ly = matrix.len();
    let lx = matrix[0].len();

    let mut aux = vec![vec![Aux::default(); lx]; ly];

    fn good_ways(matrix: &Vec<Vec<i32>>, y: usize, x: usize) -> Vec<(usize, usize)> {
        let ly = matrix.len();
        let lx = matrix[0].len();

        let mut res = vec![];
        let val = matrix[y][x];
        if x > 0 && matrix[y][x - 1] > val {
            res.push((y, x - 1));
        }
        if y > 0 && matrix[y - 1][x] > val {
            res.push((y - 1, x));
        }
        if x < lx - 1 && matrix[y][x + 1] > val {
            res.push((y, x + 1));
        }
        if y < ly - 1 && matrix[y + 1][x] > val {
            res.push((y + 1, x));
        }

        res
    }

    fn len_recur(matrix: &Vec<Vec<i32>>, aux: &mut Vec<Vec<Aux>>, y: usize, x: usize) -> usize {
        if aux[y][x] > 0 {
            return aux[y][x];
        }
        let res = good_ways(matrix, y, x)
            .into_iter()
            .map(|(next_y, next_x)| len_recur(matrix, aux, next_y, next_x))
            .max()
            .unwrap_or(0)
            + 1;
        aux[y][x] = res;
        res
    }

    // (0..ly)
    //     .map(|y| {
    //         (0..lx)
    //             .map(|x| len_recur(&matrix, &mut aux, y, x))
    //             .max()
    //             .unwrap()
    //     })
    //     .max()
    //     .unwrap() as i32

    (0..ly)
        .cartesian_product(0..lx)
        .map(|(y, x)| len_recur(&matrix, &mut aux, y, x))
        .max()
        .unwrap() as i32
}

pub fn print(matrix: &Vec<Vec<i32>>) {
    matrix.iter().for_each(|row| {
        row.iter().for_each(|x| {
            print!("{:2} ", x);
        });
        println!("");
    });
}

fn sample(n: usize) {
    let mut rng = thread_rng();

    let mut sample = vec![];
    for _i in 0..n {
        let mut row = vec![];
        for _j in 0..n {
            let x = rng.gen_range(0..(n as i32));
            row.push(x);
        }
        sample.push(row);
    }
    print(&sample);
    println!("{}", longest_increasing_path(sample))
}

fn main() {
    sample(10);
}
