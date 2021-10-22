use rand::*;
use std::collections::{BinaryHeap, HashMap};

struct EndlessBoard {
    values: HashMap<(u32, u32), i32>,
    rng: rand::prelude::ThreadRng,
}

impl EndlessBoard {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
            rng: thread_rng(),
        }
    }

    fn get(&mut self, (x, y): (u32, u32)) -> i32 {
        if let Some(&old_val) = self.values.get(&(x, y)) {
            old_val
        } else {
            let new_val = self.generate();
            self.values.insert((x, y), new_val);
            new_val
        }
    }

    fn generate(&mut self) -> i32 {
        // Сложность поиска (и само существование) пути зависят от соотношения этих чисел (ещё с числом 100 далее в тексте)
        self.rng.gen_range(-11i32..5)
    }
}

struct Arrival {
    total: i32,
    from: char,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct PosWithPriority {
    total: i32,
    from: char,
    // Для позиции, наверное, стоило выделить структуру
    pos: (u32, u32),
}

// Тащить 3 мутабельных объекта по ссылке -- это, конечно, уродливо.
// Может быть лучше не выделять эту функцию, а всё это вставить в main ?
fn find_way(
    board: &mut EndlessBoard,
    arrivals: &mut HashMap<(u32, u32), Arrival>,
    queue: &mut BinaryHeap<PosWithPriority>,
    cur_pos: PosWithPriority,
) -> i32 {
    if arrivals
        .entry(cur_pos.pos)
        .or_insert(Arrival {
            total: cur_pos.total,
            from: cur_pos.from,
        })
        .total
        > cur_pos.total
    {
        return i32::MIN;
    }

    let total = cur_pos.total + board.get(cur_pos.pos);
    // Не класть в очередь, если total меньше сколько-то? Непонятно, как узнать порог
    queue.push(PosWithPriority {
        pos: (cur_pos.pos.0 + 1, cur_pos.pos.1),
        from: 'x',
        total,
    });
    queue.push(PosWithPriority {
        pos: (cur_pos.pos.0, cur_pos.pos.1 + 1),
        from: 'y',
        total,
    });
    total
}

fn main() {
    let mut board: EndlessBoard = EndlessBoard::new();

    let mut arrivals: HashMap<(u32, u32), Arrival> = HashMap::new();
    let mut queue: BinaryHeap<PosWithPriority> = BinaryHeap::new();

    queue.push(PosWithPriority {
        pos: (0, 0),
        from: '?',
        total: 0,
    });
    let mut res: (u32, u32);
    loop {
        let next_pos = queue.pop().unwrap();
        if find_way(&mut board, &mut arrivals, &mut queue, next_pos) >= 100 {
            res = next_pos.pos;
            break;
        }
        // Проверить размер очереди и устроить ей подрезание??
    }

    let mut path: Vec<((u32, u32), i32)> = vec![];
    // Можно ли переписать, используя итераторы, так чтобы
    //  1) избавиться от mut res
    //  2) чтобы вместо push было collect в конце
    while res != (0, 0) {
        let next_res = if arrivals.get(&res).unwrap().from == 'x' {
            (res.0 - 1, res.1)
        } else {
            (res.0, res.1 - 1)
        };
        let val = board.get(res);
        path.push((res, val));
        res = next_res;
    }
    path.reverse();
    println!("{:?}", path);
}
