use std::collections::HashMap;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const D: usize = 3;
const COLOURS: usize = 4;

struct State {
    pos: [[(u32, u32); COLOURS as usize]; 2],
}

impl State {
    fn render(&self) {}
}

fn main() {
    let mut positions: Vec<(i32, i32)> = Vec::new();
    for col in 0..=10 {
        positions.push((0, col));
    }
    for col in [2, 4, 6, 8] {
        positions.push((1, col));
        positions.push((2, col));
    }

    let mut connections: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

    for outer in positions.clone() {
        for inner in positions.clone() {
            if outer == inner {
                continue;
            }
            connections.entry(outer).or_insert(Vec::new()).push(inner);
        }
    }
}
