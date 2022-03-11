use std::ops::{Add, Sub};
use rand::Rng;

const PAGE_SIZE:usize = 50; // 8196;

#[derive(Clone, Copy)]
struct BackwardOffset(u16);

#[derive(Clone, Copy)]
struct Length(u16);

impl From<BackwardOffset> for usize {
    fn from(item: BackwardOffset) -> Self {
        let dist: usize = item.0.into();
        PAGE_SIZE - dist - 1
    }
}

impl Add<Length> for usize {
    type Output = usize;

    fn add(self, rhs: Length) -> Self::Output {
        self + rhs.0 as usize
    }

}

impl Add<Length> for BackwardOffset {
    type Output = usize;
    fn add(self, rhs: Length) -> Self::Output {
        (self.0 - rhs.0) as usize
    }
}

impl Sub<BackwardOffset> for BackwardOffset {
    type Output = i8;
    fn sub(self, rhs: BackwardOffset) -> Self::Output {
        ((self.0 - rhs.0 > 0) as i8) * 2 - 1 // as usize 
    }

}

impl Add<BackwardOffset> for usize {
    type Output = usize;

    fn add(self, rhs: BackwardOffset) -> Self::Output {
        self + PAGE_SIZE - 1 - rhs.0 as usize
    }

}

// impl Add<i8> for usize {
//     type Output = usize;
//     fn add(self, rhs: i8) -> Self::Output {
//         (self as i8 - rhs) as usize
//     }
// }


#[derive(Clone)]
struct DataBase {
    //array: [u8; PAGE_SIZE],
    array: Vec<u8>,
}

impl Default for DataBase {
    fn default() -> Self {
        let array:Vec<u8> = vec![0;PAGE_SIZE];
        Self {
            array,
        }
    }
}

impl DataBase {
    pub fn push(&mut self, add: u8) {
        self.array.push(add);
    }
    pub fn remove(&mut self, number: usize) {
        self.array.remove(number);
    }
    pub fn get(&self, number: usize) -> u8 {
        return self.array[number];
    }
    pub fn print(&self) {
        println!("{:?}", self.array);
    }
    pub fn randoming(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.array.len() {
            self.array[i] = rng.gen_range(0..10);
        }
    }
    pub fn move_row(&mut self, old_pos: BackwardOffset, new_pos: BackwardOffset, len: Length) {
        // (
        //     &mut self, 
        //     //buf: &mut [u8; PAGE_SIZE],
        //     //buf: &mut DataBase,
        //     old_pos: BackwardOffset,
        //     new_pos: BackwardOffset,
        //     len: Length,
        // )
        
        let old_offset:usize = old_pos.into();
        let next = old_offset + len;
        let next_pos:usize = new_pos.into();

        // clone
        let save = self.array[old_offset..next].to_vec();

        // move
        if old_offset<next_pos {
            for i in old_offset..next_pos {
                self.array[i] = self.array[i+len];
            }
        }
        else {
            let mut i = old_offset-1;
            while i!=next_pos-1 {
                self.array[i+len] = self.array[i];
                i-=1;
            }
        }

        // for i in 3..1 {
        //     println!("{}",i);
        // }

        // add
        for i in 0..save.len() {
            self.array[i+new_pos] = save[i];
        }
    } 
}

fn main() {
    print!("create array:                ");
    let mut data = DataBase::default();
    data.print();
    print!("write random data in array:  ");
    data.randoming();
    data.print();
    print!("add new element array:       ");
    data.push(23);
    data.print();
    print!("get last element array:      ");
    println!("{}",data.get(PAGE_SIZE));
    print!("del last element array:      ");
    data.remove(PAGE_SIZE);
    data.print();
    print!("move data in array:          ");
    data.move_row(BackwardOffset(2), BackwardOffset(5), Length(2));
    data.print();

}