use regex::Regex;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Block {
    dim_range: [(i32, i32); 3],
}

fn push_check(vec: &mut Vec<Block>, block: Block) {
    println!("{:?}", block);

    if false
        || block.dim_range[0].0 > block.dim_range[0].1
        || block.dim_range[1].0 > block.dim_range[1].1
        || block.dim_range[2].0 > block.dim_range[2].1
    {
        println!("here");
    }

    vec.push(block);
}

fn dim_slice(victim: Block, dim: usize, range: (i32, i32)) -> Vec<Block> {
    let mut ret = Vec::new();

    let victim_dim_range = victim.dim_range[dim];

    println!("enter dim_slice");

    if victim_dim_range.1 < range.0
        || victim_dim_range.0 > range.1
        || victim_dim_range.0 > range.0 && victim_dim_range.1 < range.1
    {
        println!("dim_slice a");
        push_check(&mut ret, victim);
    } else if victim_dim_range.0 < range.0 && victim_dim_range.1 > range.1 {
        println!("dim_slice b");
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
        println!("dim_slice c");
        let mut l = victim;
        l.dim_range[dim].1 = range.1;
        push_check(&mut ret, l);

        let mut r = victim;
        r.dim_range[dim].0 = range.1 + 1;
        push_check(&mut ret, r);
    } else if victim_dim_range.0 < range.0 {
        println!("dim_slice d");
        let mut l = victim;
        l.dim_range[dim].1 = range.0 - 1;
        push_check(&mut ret, l);

        let mut r = victim;
        r.dim_range[dim].0 = range.0;
        push_check(&mut ret, r);
    } else {
        println!("dim_slice e");
        push_check(&mut ret, victim);
    }

    println!("leave dim_slice");
    ret
}

fn point_in_block(x: i32, y: i32, z: i32, block: Block) -> bool {
    return true
        && x >= block.dim_range[0].0
        && x <= block.dim_range[0].1
        && y >= block.dim_range[1].0
        && y <= block.dim_range[1].1
        && z >= block.dim_range[2].0
        && z <= block.dim_range[2].1;
}

fn is_overlapped(is_this: Block, overlapped_with: Block) -> bool {
    return false
        || point_in_block(
            is_this.dim_range[0].0,
            is_this.dim_range[1].0,
            is_this.dim_range[2].0,
            overlapped_with,
        )
        || point_in_block(
            is_this.dim_range[0].0,
            is_this.dim_range[1].0,
            is_this.dim_range[2].1,
            overlapped_with,
        )
        || point_in_block(
            is_this.dim_range[0].0,
            is_this.dim_range[1].1,
            is_this.dim_range[2].0,
            overlapped_with,
        )
        || point_in_block(
            is_this.dim_range[0].0,
            is_this.dim_range[1].1,
            is_this.dim_range[2].1,
            overlapped_with,
        )
        || point_in_block(
            is_this.dim_range[0].1,
            is_this.dim_range[1].0,
            is_this.dim_range[2].0,
            overlapped_with,
        )
        || point_in_block(
            is_this.dim_range[0].1,
            is_this.dim_range[1].0,
            is_this.dim_range[2].1,
            overlapped_with,
        )
        || point_in_block(
            is_this.dim_range[0].1,
            is_this.dim_range[1].1,
            is_this.dim_range[2].0,
            overlapped_with,
        )
        || point_in_block(
            is_this.dim_range[0].1,
            is_this.dim_range[1].1,
            is_this.dim_range[2].1,
            overlapped_with,
        );
}

fn block_slice(victim: Block, block: Block) -> Vec<Block> {
    if !is_overlapped(victim, block) {
        let mut ret = Vec::new();

        push_check(&mut ret, victim);
        return ret;
    }

    let a = dim_slice(victim, 0, block.dim_range[0]);

    let mut b = Vec::new();
    for victim in a.clone() {
        b.extend(dim_slice(victim, 1, block.dim_range[1]));
    }

    let mut c = Vec::new();
    for victim in b.clone() {
        c.extend(dim_slice(victim, 2, block.dim_range[2]));
    }

    c
}

fn slice(state: &Vec<Block>, block: Block) -> Vec<Block> {
    let mut ret = Vec::new();

    for scan in state {
        let copy = block.clone();
        ret.extend(block_slice(*scan, copy));
    }

    ret
}

fn is_in(is_this: Block, in_this: Block) -> bool {
    return true
        && is_this.dim_range[0].0 >= in_this.dim_range[0].0
        && is_this.dim_range[0].1 <= in_this.dim_range[0].1
        && is_this.dim_range[1].0 >= in_this.dim_range[1].0
        && is_this.dim_range[1].1 <= in_this.dim_range[1].1
        && is_this.dim_range[2].0 >= in_this.dim_range[2].0
        && is_this.dim_range[2].1 <= in_this.dim_range[2].1;
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

        if state.len() == 0 {
            push_check(&mut state, block);
        } else {
            state = slice(&state, block);
            let mut filtered = Vec::new();
            for elem in state.clone() {
                if !is_in(elem, block) {
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

    let mut sum = 0u64;
    for block in state {
        println!(
            "{} {} {} {} {} {}",
            block.dim_range[0].1,
            block.dim_range[0].0,
            block.dim_range[1].1,
            block.dim_range[1].0,
            block.dim_range[2].1,
            block.dim_range[2].0
        );

        let xl = (block.dim_range[0].1 - block.dim_range[0].0) as u64;
        let yl = (block.dim_range[1].1 - block.dim_range[1].0) as u64;
        let zl = (block.dim_range[2].1 - block.dim_range[2].0) as u64;

        println!("{} {} {}", xl, yl, zl);

        sum += xl * xl + yl * yl + zl * zl;
    }

    println!("{}", sum);
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
