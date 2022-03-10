use std::ops::{Add, Sub};
use rand::Rng;

//const PAGE_SIZE:usize = 8196;
const PAGE_SIZE:usize = 50;


#[derive(Clone, Copy)]
struct BackwardOffset(u16);

#[derive(Clone, Copy)]
struct Length(u16);


// impl From<BackwardOffset> for usize {
//     fn from(item: BackwardOffset) -> Self {
//         let dist: usize = item.0.into();
//         PAGE_SIZE - dist
//     }
// }

impl From<BackwardOffset> for usize {
    fn from(item: BackwardOffset) -> Self {
        let dist: usize = item.0.into();
        PAGE_SIZE - dist
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

// impl Add<BackwardOffset> for BackwardOffset {
//     type Output = usize;
//     fn add(self, rhs: BackwardOffset) -> Self::Output {
//         (self.0 + rhs.0) as usize
//     }
// }

impl Sub<BackwardOffset> for BackwardOffset {
    type Output = usize;
    fn sub(self, rhs: BackwardOffset) -> Self::Output {
        (self.0 - rhs.0) as usize 
    }

}


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
        let next_pos = new_pos - old_pos;

        println!("{} {} {}", old_offset, next, next_pos);

        for i in old_offset..next {
            let t = self.array[i+next_pos];
            self.array[i+next_pos] = self.array[i];
            self.array[i] = t;
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
    data.move_row(BackwardOffset(3), BackwardOffset(4), Length(2));
    data.print();

}