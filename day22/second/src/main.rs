use regex::Regex;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Block {
    dim_range: [(i32, i32); 3],
}

fn push_check(vec: &mut Vec<Block>, block: Block) {
    if false {
        println!("{:?}", block);
    }

    if false
        || block.dim_range[0].0 > block.dim_range[0].1
        || block.dim_range[1].0 > block.dim_range[1].1
        || block.dim_range[2].0 > block.dim_range[2].1
    {
        assert!(false);
    }

    vec.push(block);
}

fn dim_slice(victim: Block, dim: usize, range: (i32, i32)) -> Vec<Block> {
    let mut ret = Vec::new();

    let victim_dim_range = victim.dim_range[dim];

    if victim_dim_range.1 < range.0
        || victim_dim_range.0 > range.1
        || victim_dim_range.0 > range.0 && victim_dim_range.1 < range.1
    {
        push_check(&mut ret, victim);
    } else if victim_dim_range.0 < range.0 && victim_dim_range.1 > range.1 {
        let mut l = victim;

        l.dim_range[dim].1 = range.0 - 1;
        push_check(&mut ret, l);

        let mut m = victim;
        m.dim_range[dim].0 = range.0;
        m.dim_range[dim].1 = range.1;
        push_check(&mut ret, m);

        let mut r = victim;
        r.dim_range[dim].0 = range.1 + 1;
        push_check(&mut ret, r);
    } else if victim_dim_range.1 > range.1 {
        let mut l = victim;
        l.dim_range[dim].1 = range.1;
        push_check(&mut ret, l);

        let mut r = victim;
        r.dim_range[dim].0 = range.1 + 1;
        push_check(&mut ret, r);
    } else if victim_dim_range.0 < range.0 {
        let mut l = victim;
        l.dim_range[dim].1 = range.0 - 1;
        push_check(&mut ret, l);

        let mut r = victim;
        r.dim_range[dim].0 = range.0;
        push_check(&mut ret, r);
    } else {
        push_check(&mut ret, victim);
    }

    ret
}

fn b_fully_contains_a(a: Block, b: Block) -> bool {
    a.dim_range[0].0 >= b.dim_range[0].0
        && a.dim_range[0].1 <= b.dim_range[0].1
        && a.dim_range[1].0 >= b.dim_range[1].0
        && a.dim_range[1].1 <= b.dim_range[1].1
        && a.dim_range[2].0 >= b.dim_range[2].0
        && a.dim_range[2].1 <= b.dim_range[2].1
}

fn point_in_block(x: i32, y: i32, z: i32, block: Block) -> bool {
    x >= block.dim_range[0].0
        && x <= block.dim_range[0].1
        && y >= block.dim_range[1].0
        && y <= block.dim_range[1].1
        && z >= block.dim_range[2].0
        && z <= block.dim_range[2].1
}

fn has_corners_in(a: Block, b: Block) -> bool {
    point_in_block(a.dim_range[0].0, a.dim_range[1].0, a.dim_range[2].0, b)
        || point_in_block(a.dim_range[0].0, a.dim_range[1].0, a.dim_range[2].1, b)
        || point_in_block(a.dim_range[0].0, a.dim_range[1].1, a.dim_range[2].0, b)
        || point_in_block(a.dim_range[0].0, a.dim_range[1].1, a.dim_range[2].1, b)
        || point_in_block(a.dim_range[0].1, a.dim_range[1].0, a.dim_range[2].0, b)
        || point_in_block(a.dim_range[0].1, a.dim_range[1].0, a.dim_range[2].1, b)
        || point_in_block(a.dim_range[0].1, a.dim_range[1].1, a.dim_range[2].0, b)
        || point_in_block(a.dim_range[0].1, a.dim_range[1].1, a.dim_range[2].1, b)
}

fn a_and_b_overlap(a: Block, b: Block) -> bool {
    has_corners_in(a, b) || has_corners_in(b, a)
}

fn block_slice(victim: Block, block: Block) -> Vec<Block> {
    if a_and_b_overlap(victim, block) {
        let a = dim_slice(victim, 0, block.dim_range[0]);

        let mut b = Vec::new();
        for victim in a {
            b.extend(dim_slice(victim, 1, block.dim_range[1]));
        }

        let mut c = Vec::new();
        for victim in b {
            c.extend(dim_slice(victim, 2, block.dim_range[2]));
        }

        c
    } else {
        let mut ret = Vec::new();

        ret.push(victim);
        ret
    }
}

fn volume(state: &Vec<Block>) -> u64 {
    let mut sum = 0u64;
    for block in state {
        if false {
            println!(
                "{} {} {} {} {} {}",
                block.dim_range[0].1,
                block.dim_range[0].0,
                block.dim_range[1].1,
                block.dim_range[1].0,
                block.dim_range[2].1,
                block.dim_range[2].0
            );
        }

        let xl = (block.dim_range[0].1 - block.dim_range[0].0 + 1) as u64;
        let yl = (block.dim_range[1].1 - block.dim_range[1].0 + 1) as u64;
        let zl = (block.dim_range[2].1 - block.dim_range[2].0 + 1) as u64;

        if false {
            println!("{} {} {}", xl, yl, zl);
        }

        sum += xl * yl * zl;
    }

    sum
}

fn slice(state: &Vec<Block>, block: Block) -> Vec<Block> {
    let mut ret = Vec::new();

    let before = volume(state);

    for scan in state {
        let sliced = block_slice(*scan, block);

        ret.extend(sliced);
    }

    let after = volume(&ret);

    if before != after {
        println!("{}", before);
        println!("{}", after);
        println!();
        assert!(false);
    }

    ret
}

fn main() {
    let stdin = io::stdin();
    let mut state: Vec<Block> = Vec::new();

    for (index, line) in stdin.lock().lines().enumerate() {
        let line = line.expect("Could not read line from standard in");
        let line = line.as_str();

        let r = Regex::new(r"(off|on) x=([-+0-9]*)\.\.([-+0-9]*),y=([-+0-9]*)\.\.([-+0-9]*),z=([-+0-9]*)\.\.([-+0-9]*)").unwrap();
        let caps = r.captures(line).unwrap();
        let on_off = &caps[1];
        let xl = caps[2].parse::<i32>().unwrap();
        let xu = caps[3].parse::<i32>().unwrap();
        let yl = caps[4].parse::<i32>().unwrap();
        let yu = caps[5].parse::<i32>().unwrap();
        let zl = caps[6].parse::<i32>().unwrap();
        let zu = caps[7].parse::<i32>().unwrap();

        let block = Block {
            dim_range: [(xl, xu), (yl, yu), (zl, zu)],
        };

        if state.is_empty() && on_off == "on" {
            push_check(&mut state, block);
        } else {
            state = slice(&state, block);
            let mut filtered = Vec::new();
            for elem in state {
                if b_fully_contains_a(elem, block) {
                } else if has_corners_in(elem, block) {
                    assert!(false);
                } else if has_corners_in(block, elem) {
                    assert!(false);
                } else {
                    push_check(&mut filtered, elem);
                }
            }
            state = filtered;
            if on_off == "on" {
                push_check(&mut state, block);
            }
        }
        println!("{} {}", index, state.len());
    }

    println!("{}", volume(&state));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dim_slice() {
        let b = Block {
            dim_range: [(-3, 3), (1, 1), (1, 1)],
        };
        assert!(
            dim_slice(b, 0, (4, 5))
                == [Block {
                    dim_range: [(-3, 3), (1, 1), (1, 1)]
                }]
        );
        assert!(
            dim_slice(b, 0, (-5, -4))
                == [Block {
                    dim_range: [(-3, 3), (1, 1), (1, 1)]
                }]
        );
        assert!(
            dim_slice(b, 0, (-4, 4))
                == [Block {
                    dim_range: [(-3, 3), (1, 1), (1, 1)]
                }]
        );
        assert!(
            dim_slice(b, 0, (-2, 2))
                == [
                    Block {
                        dim_range: [(-3, -3), (1, 1), (1, 1)]
                    },
                    Block {
                        dim_range: [(-2, 2), (1, 1), (1, 1)]
                    },
                    Block {
                        dim_range: [(3, 3), (1, 1), (1, 1)]
                    }
                ]
        );
        assert!(
            dim_slice(b, 0, (-4, -3))
                == [
                    Block {
                        dim_range: [(-3, -3), (1, 1), (1, 1)]
                    },
                    Block {
                        dim_range: [(-2, 3), (1, 1), (1, 1)]
                    }
                ]
        );
        assert!(
            dim_slice(b, 0, (-2, 5))
                == [
                    Block {
                        dim_range: [(-3, -3), (1, 1), (1, 1)]
                    },
                    Block {
                        dim_range: [(-2, 3), (1, 1), (1, 1)]
                    }
                ]
        );
        assert!(
            dim_slice(b, 0, (-3, 3))
                == [Block {
                    dim_range: [(-3, 3), (1, 1), (1, 1)]
                }]
        );
    }
}
