use std::ops::{Index, IndexMut};
use rand::*;


const S: usize = 8;
struct Board([i32; S * S]);

impl Index<(usize, usize)> for Board {
    type Output = i32;
    fn index(&self, (x, y): (usize, usize)) -> &i32 {
        &self.0[y * S + x]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut i32 {
        &mut self.0[y * S + x]
    }
}

impl Index<Pos> for Board {
    type Output = i32;
    fn index(&self, Pos{x, y}: Pos) -> &i32 {
        &self.0[y * S + x]
    }
}

impl IndexMut<Pos> for Board {
    fn index_mut(&mut self, p: Pos) -> &mut i32 {
        &mut self.0[p.y * S + p.x]
    }
}

impl Index<usize> for Board {
    type Output = [i32];
    fn index(&self, y: usize) -> &Self::Output {
        &self.0[(y * S)..(y * S + S)]
    }
}
/////////////////////////////////////////////////
// struct Pos((usize, usize));
// Pos((1, 2))
#[derive(Clone, Copy, Default)]
struct Pos {
    x: usize,
    y: usize
}
// Pos{ x: 1, y: 2}

impl Pos {
    // fn x(&self) -> usize {
    //     self.0.0
    // }
    // fn y(&self) -> usize {
    //     self.0.1
    // }
    fn moves(&self) -> Vec<Pos> {
        let mut res = Vec::new();
        if self.x < S - 1 && self.y < S - 1{
            res.push(Pos{x: self.x + 1, y: self.y + 1});
        }
        if self.x < S - 1 {
            res.push(Pos{x: self.x + 1, y: self.y});
        }
        if self.y < S - 1 {
            res.push(Pos{x: self.x, y: self.y + 1});
        }
        res
    }
}

impl Board {
    fn new_empty() -> Self {
        Self([0; S * S])
    }
    fn new_random() -> Self {
        let mut res = Self::new_empty();
        let mut rng = thread_rng();
        for y in 0..8 {
            for x in 0..8 {
                if (x, y) != (0, 0) && (x, y) != (S - 1, S - 1) {
                    let diag_dist = i32::abs(x as i32 - y as i32);
                    let r = rng.gen_range(0..(10 - diag_dist)) - S as i32 / 2;
                    res[Pos{x, y}] = r;
                }
            }
        }
        res
    }
}

fn print_board(b: &Board) {
    for y in (0..=7).rev() {
        for x in 0..=7 {
            print!("{:+2} ", b[Pos{x, y}]);
        }
        println!("\n");
    }
}

fn print_board_with_moves(b: &Board, moves: &Board) {
    for y in (0..=7).rev() {
        for x in 0..=7 {
            let m = moves[Pos{x, y}];
            if m == 8 {
                print!(" | ");
            } else if m == 9 {
                print!("  /");
            } else {
                print!("   ");
            } 
        }
        println!();
        for x in 0..=7 {
            print!("{:+2}{}", b[Pos{y, x}], if moves[Pos{x, y}] == 1 {'~'} else {' '});
        }
        println!();
    }
}

fn path_to_board(p: &[Pos]) -> Board {
    let mut res = Board::new_empty();
    let mut prev_pos: Option<Pos> = None;
    for &cur_pos in p.iter() {
        if let Some(prev_pos) = prev_pos {
            if prev_pos.x == cur_pos.x + 1 && prev_pos.y == cur_pos.y + 1 {
                res[cur_pos] = 9
            } else if prev_pos.x == cur_pos.x + 1 {
                res[cur_pos] = 1
            } else if prev_pos.y == cur_pos.y + 1 {
                res[cur_pos] = 8
            }
        }
        prev_pos = Some(cur_pos);
    }
    res
}

fn is_odd(x: &i32) -> bool {
    x % 2 == 1
}

fn dumb_path(b: &Board) -> Vec<Pos> {
    ((0..S).rev()).zip((0..S).rev()).map(|(x, y)| Pos{x, y}).collect()
}
// fn dumb_path2(b: &Board) -> Vec<Pos> {
//     let mut res = vec![];
//     for i in 0..S {
//         res.push((i, i))
//     }
//     res
// }

fn smart_path(b: &Board, cur_pos: Pos) -> (i32, Vec<Pos>) {
    let this_value = b[cur_pos];
    let best = cur_pos.moves().into_iter().map(|next|{
        smart_path(b, next)
    }).max_by_key(|&(score, _)|score);

    // let mut best = -999;
    // let mut best_path = vec![];
    // for next in cur_pos.moves() {
    //     let (next_score, next_path) = smart_path(b, next);
    //     if next_score > best {
    //         best = next_score;
    //         best_path = next_path;
    //     }
    // }
    // let this_value = b[cur_pos];
    if let Some((best, mut best_path)) = best {
        best_path.push(cur_pos);
        (best + this_value, best_path)
    } else {
        (this_value, vec![cur_pos])
    }
}

fn main() {
    let b = Board::new_random();
    // print_board(&b);
    let path = smart_path(&b, Pos{x: 0, y: 0}).1;
    print_board_with_moves(&b, &path_to_board(&path));
}