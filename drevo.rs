use std::{collections::HashMap, hash::Hash};

struct Entry<Key, Value> {
    key: Key,
    value: Value,
    next: Option<usize>,
    prev: Option<usize>,
}

struct Cache<Key: Eq + Hash, Value> {
    n: usize,

    // m: HashMap<Key, ???>,
    // l: List<Entry>,
    m: HashMap<Key, usize>,
    v: Vec<Entry<Key, Value>>,
    list_head: Option<usize>,
    list_tail: Option<usize>,
}

// LFU least frequently used
// LRU last recently used

// [abc] [jkl] [ghi] [def] [mno]
// [ghi] -> [abc] [jkl]--> <--[def] [mno]
// [abc] [jkl] [ghi] [def] x[mno]x

//
// [abc] [jkl] [ghi] [def] [mno]
//

// id Val Prev Next
// 0 (abc, NULL, 4)
// 1 (def, 2, 3)
// 2 (ghi, 4, 1)
// 3 (mno, 1, NULL)
// 4 (jkl, 0, 2)

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new(n: usize) -> Self {
        Cache {
            n,
            m: HashMap::with_capacity(n),
            v: Vec::with_capacity(n),
            list_head: None,
            list_tail: None,
        }
    }

    fn move_to_head(&mut self, pos: usize) {
        if let Some(old_prev_pos) = self.v[pos].prev {
            self.v[old_prev_pos].prev = self.v[pos].next;
        } else {
            // Он уже первый
            return;
        }
        if let Some(old_next_pos) = self.v[pos].next {
            self.v[old_next_pos].prev = self.v[pos].prev;
        } else {
            self.list_tail = self.v[pos].prev;
        }

        self.v[pos].next = self.list_head;
        self.v[pos].prev = None;
        self.list_head = Some(pos);
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.v.len() >= self.n {
            todo!()
        } else if let Some(&pos) = self.m.get(&key) {
            self.v[pos].value = value;
            self.move_to_head(pos);
        } else {
            let new_pos = self.v.len();

            if let Some(head) = self.list_head {
                self.v[head].prev = Some(new_pos);
            } else {
                self.list_tail = Some(new_pos);
            }

            self.v.push(Entry {
                key: key.clone(),
                value,
                next: self.list_head,
                prev: None,
            });
            self.list_head = Some(new_pos);
            assert!(self.m.insert(key, new_pos).is_none())
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        self.m.get(key).map(|&pos| self.v[pos].value.clone())
    }
}

#[derive(Clone)]
struct Payload {
    s: String,
}

fn main() {
    let mut cache = Cache::<_, Payload>::new(1000);
    cache.insert(
        3u32,
        Payload {
            s: "mom".to_string(),
        },
    );
    assert!(cache.get(&3).unwrap().s == "mom");

    println!("Hello, world!");
}