use std::{collections::HashMap, hash::Hash};

use rand::{prelude::ThreadRng, thread_rng, Rng};

#[derive(Debug)]
struct Entry<Key: Eq + Hash, Value> {
    key: Key,
    value: Value,
    goodness: u32,
    generation: u32,
}

// abc   def   ghi
//  10    10    7
//              ^

struct Cache<Key: Eq + Hash, Value> {
    capacity: usize,
    hand: usize,
    current_generation: u32,
    // time: usize,
    m: HashMap<Key, usize>,
    clock: Vec<Entry<Key, Value>>,
    // rng: ThreadRng,
}

const STARTING_GOODNESS: u32 = 10;
const ROUND_PENALTY: u32 = 1;

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            hand: 0,
            m: HashMap::with_capacity(capacity),
            clock: vec![],
            // rng: thread_rng(),
            current_generation: 1,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.m.len() < self.capacity {
            self.clock.push(Entry {
                key: key.clone(),
                value,
                goodness: STARTING_GOODNESS,
                generation: 0,
            });
            self.m.insert(key, self.clock.len() - 1);
        } else {
            let victim = self.find_victim();
            assert!(self.m.remove(&self.clock[victim].key).is_some());
            self.clock[victim] = Entry {
                key: key.clone(),
                value,
                goodness: STARTING_GOODNESS,
                generation: 0,
            };
            self.m.insert(key, victim);
        }
    }

    fn find_victim(&mut self) -> usize {
        // self.time += 1;
        // return self.rng.gen_range(0..self.capacity);

        let mut threshold = 1;
        loop {
            self.hand += 1;
            if self.hand >= self.capacity {
                self.hand = 0;
            }

            if self.clock[self.hand].generation == self.current_generation {
                continue;
            }
            if self.clock[self.hand].goodness < threshold {
                return self.hand;
            } else {
                self.clock[self.hand].goodness =
                    self.clock[self.hand].goodness.saturating_sub(ROUND_PENALTY);
            }
            threshold += 1;
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<V> {
        self.m.get(key).map(|&pos| {
            let element = &mut self.clock[pos];
            element.goodness += 1;
            element.value.clone()
        })
    }

    pub fn print_all(&self)
    where
        K: std::fmt::Debug,
        V: std::fmt::Debug,
    {
        self.clock.iter().for_each(|el| println!("{:?}", el))
    }
}

#[derive(Clone, Debug)]
struct Payload {
    s: String,
}

const N: usize = 100;

fn main() {
    let mut cache = Cache::<_, Payload>::new(N);
    let mut rng = thread_rng();

    let mut miss_count = 0;
    let m = 3 * N as u32;
    for _ in 0..N * 10000 {
        let x_final;
        loop {
            let r = rng.gen_range(0..m * m);
            let (x, y) = (r / m as u32, r % m);
            if x <= y {
                x_final = x;
                break;
            }
        }
        if cache.get_mut(&x_final).is_none() {
            miss_count += 1;
            cache.insert(
                x_final,
                Payload {
                    s: x_final.to_string(),
                },
            );
        }
    }

    cache.print_all();
    println!("miss={}", miss_count);
}
