use std::collections::HashMap;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const D: usize = 3;
const COLOURS: usize = 4;

struct State {
    pos: [[(u32, u32); COLOURS]; 2],
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
            connections.entry(outer).or_default().push(inner);
        }
    }

    let mut connections: HashMap<(i32, i32), [Vec<((i32, i32), i32)>; COLOURS]> = HashMap::new();

    for col in 0..=10 {
        if positions.contains(&(1, col)) {
            for colour in 0..COLOURS {
                let mut l: Option<(i32, i32)> = Some((0, col));
                let mut r: Option<(i32, i32)> = Some((0, col));
                let mut d: Option<(i32, i32)> = None;

                loop {
                    if let Some(pos) = l {
                        let mut l_col = pos.1 - 1;

                        if d.is_none() && l_col == 2 + colour as i32 * 2 {
                            d = Some((1, l_col));
                        } else {
                            d = match d {
                                Some((1, c)) => Some((2, c)),
                                Some((2, _)) => None,
                                _ => None,
                            }
                        }

                        l_col = match l_col {
                            2 | 4 | 6 | 8 => l_col - 1,
                            _ => l_col,
                        };
                        if l_col < 0 {
                            l = None;
                        }
                    }
                }
            }
        }
    }
}
