use regex::Regex;
use std::cmp;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Block {
    dim_range: [(i32, i32); 3],
}

fn dim_slice(victim: Block, dim: usize, range: (i32, i32)) -> Vec<Block> {
    let mut ret = Vec::new();

    let victim_dim_range = victim.dim_range[dim];

    println!("victim_dim_range {:?}", victim_dim_range);
    println!("range {:?}", range);

    if victim_dim_range.1 < range.0 {
        println!("a");
        ret.push(victim);
    } else if victim_dim_range.0 > range.1 {
        println!("b");
        ret.push(victim);
    } else if victim_dim_range.0 > range.0 && victim_dim_range.1 < range.1 {
        println!("c");
        ret.push(victim);
    } else if victim_dim_range.0 < range.0 && victim_dim_range.1 > range.1 {
        println!("d");
        let mut l = victim;

        l.dim_range[dim].1 = range.0 - 1;
        ret.push(l);

        let mut m = victim;
        m.dim_range[dim].0 = range.0;
        m.dim_range[dim].1 = range.1;
        ret.push(m);

        let mut r = victim;
        r.dim_range[dim].0 = range.1 + 1;
        ret.push(r);
    } else if victim_dim_range.1 > range.1 {
        println!("e");
        let mut l = victim;
        l.dim_range[dim].1 = range.1 - 1;
        ret.push(l);

        let mut r = victim;
        r.dim_range[dim].0 = range.1;
        ret.push(r);
    } else if victim_dim_range.0 < range.0 {
        println!("f");
        let mut l = victim;
        l.dim_range[dim].1 = range.0 - 1;
        ret.push(l);

        let mut r = victim;
        r.dim_range[dim].0 = range.0;
        ret.push(r);
    } else {
        println!("g");
        ret.push(victim);
    }

    ret
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
                        dim_range: [(-3, -4), (1, 1), (1, 1)]
                    },
                    Block {
                        dim_range: [(-3, 3), (1, 1), (1, 1)]
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

fn block_slice(victim: Block, block: Block) -> Vec<Block> {
    let ret = Vec::new();

    ret
}

fn slice(mut state: Vec<Block>, block: Block) -> Vec<Block> {
    let mut ret = Vec::new();

    for scan in state {
        let copy = block.clone();
        ret.extend(block_slice(scan, copy));
    }

    ret
}

fn main() {
    let stdin = io::stdin();
    let state: Vec<Block> = Vec::new();

    for line in stdin.lock().lines() {
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
    }
}
