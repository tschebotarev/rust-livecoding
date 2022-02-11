#![feature(min_const_generics)]
#![feature(generic_const_exprs)]

#![feature(min_specialization)]
use std::mem::size_of;

use num::{Integer, Signed, Unsigned};
use rand::*;

type DigitType = u8;

trait GetDigit {
    fn get_digit()
}


fn get_digit<T, const S: usize, const I: bool>(x: &T, step: usize) -> DigitType {
    let a: [DigitType; S] = unsafe { std::mem::transmute_copy(x) };
    if I && step == S - 1 {
        a[step] ^ (1 << (size_of::<DigitType>() * 8 - 1))
    } else {
        a[step]
    }
}

fn radix_sort<T, const I: bool>(arr: &mut [T])
where
    T: num::Integer + Default + Clone,
    [(); size_of::<T>()]: Sized,
{
    let steps = size_of::<T>() / size_of::<DigitType>();
    let mut tmp_arr = vec![T::default(); arr.len()];
    for step in 0..steps {
        let (a_from, a_to) = if step % 2 == 0 {
            (&arr[..], &mut tmp_arr[..])
        } else {
            (&tmp_arr[..], &mut arr[..])
        };

        let mut counts = vec![0usize; 1 << (size_of::<DigitType>() * 8)];
        for x in a_from {
            let pos = get_digit::<T, { size_of::<T>() }, I>(x, step) as usize;
            counts[pos] += 1;
        }

        let mut begins = Vec::with_capacity(1 << (size_of::<DigitType>() * 8));
        begins.push(0);
        for c in counts[0..(DigitType::MAX) as usize].iter() {
            begins.push(begins.last().unwrap() + c);
        }

        for x in a_from {
            let pos = get_digit::<T, { size_of::<T>() }, I>(x, step) as usize;
            a_to[begins[pos]] = x.clone();
            begins[pos] += 1;
        }
    }

    if steps % 2 == 1 {
        for (dst, src) in arr.iter_mut().zip(tmp_arr.iter()) {
            *dst = src.clone();
        }
    }
}

// const SS: usize = size_of::<u32>() / size_of::<DigitType>();

fn main() {
    let mut rng = thread_rng();
    let mut v: Vec<_> = (0..1_000_000)
        .map(|_| rng.gen_range(i32::MIN..i32::MAX))
        .collect();

    // println!("{:?}\n***************************", v);
    radix_sort::<_, true>(&mut v[..]);
    // println!("{:?}\n", v);
    let w = v.clone();
    v.sort();
    assert_eq!(v, w);
}

// struct Point {
//     x: u32,
//     y: u32,
// }
