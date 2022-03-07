use std::ops::{Add, Sub};

const PAGE_SIZE:usize = 8196;

#[derive(Clone, Copy)]
struct BackwardOffset(u16);

#[derive(Clone, Copy)]
struct Length(u16);


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

// impl Sub<BackwardOffset> for usize {
//     type Output = usize;
//     fn sub(self, rhs: BackwardOffset) -> Self::Output {
//         self - rhs.0 as usize
//     }

// }


fn move_row(
    buf: &mut [u8; PAGE_SIZE],
    old_pos: BackwardOffset,
    new_pos: BackwardOffset,
    len: Length,
) {
    let old_offset: usize = old_pos.into();

    let next = old_offset + len;
    let next_pos = old_pos + len;

    //let x = new_pos + old_pos;

    // timestamp -- секунды от рождества христова
    // interval -- секунды со знаком

    let new_offset: usize = PAGE_SIZE - new_pos;
    let len: usize = len.0.into();
    // let len: usize = len.0 as usize;

    //buf[new_offset..new_offset + len].copy_from_slice(&buf[old_offset..old_offset + len]);

} 

fn main() {

}