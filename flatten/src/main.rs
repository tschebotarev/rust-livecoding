const N: usize = 1000;

#[allow(dead_code)]
fn sum1(v3: &Vec<Vec<Vec<u64>>>) -> u64 {
    let mut res = 0;
    for v2 in v3 {
        for v1 in v2 {
            for x in v1 {
                res += x;
            }
        }
    }
    res
}

#[allow(dead_code)]
fn sum2(v3: &Vec<Vec<Vec<u64>>>) -> u64 {
    v3.iter()
        .flat_map(|v2| v2.iter().flat_map(|v1| v1.iter()))
        .sum()
}

struct MyFlatten<O: Iterator, I: Iterator, F: Fn(O::Item) -> I> {
    outer: O,
    current_inner: Option<I>,
    converter: F,
}

impl<O: Iterator, I: Iterator, F: Fn(O::Item) -> I> Iterator for MyFlatten<O, I, F> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(cur) = &mut self.current_inner {
                let x = cur.next();
                if x.is_some() {
                    return x;
                } else {
                    self.current_inner = None;
                }
            } else {
                if let Some(inner) = self.outer.next() {
                    self.current_inner = Some((self.converter)(inner));
                } else {
                    return None;
                }
            }
        }
    }
}

fn flatten<O: Iterator, I: Iterator, F: Fn(O::Item) -> I>(o: O, f: F) -> MyFlatten<O, I, F> {
    MyFlatten {
        outer: o,
        current_inner: None,
        converter: f,
    }
}

fn sum3(v3: &Vec<Vec<Vec<u64>>>) -> u64 {
    flatten(v3.iter(), |v2| flatten(v2.iter(), |v1| v1.iter())).sum()
}

fn main() {
    let v3d = vec![vec![vec![1u64; N]; N]; N];

    let res1 = sum3(&v3d);
    println!("{}", res1)
}
