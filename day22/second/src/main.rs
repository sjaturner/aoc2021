use regex::Regex;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Block {
    dim_range: [(i32, i32); 3],
}

fn point_in_block(x: i32, y: i32, z: i32, block: Block) -> bool {
    x >= block.dim_range[0].0
        && x <= block.dim_range[0].1
        && y >= block.dim_range[1].0
        && y <= block.dim_range[1].1
        && z >= block.dim_range[2].0
        && z <= block.dim_range[2].1
}

fn b_fully_contains_a(a: Block, b: Block) -> bool {
    a.dim_range[0].0 >= b.dim_range[0].0
        && a.dim_range[0].1 <= b.dim_range[0].1
        && a.dim_range[1].0 >= b.dim_range[1].0
        && a.dim_range[1].1 <= b.dim_range[1].1
        && a.dim_range[2].0 >= b.dim_range[2].0
        && a.dim_range[2].1 <= b.dim_range[2].1
}

impl Block {
    fn overlaps(&self, other: Block) -> bool {
        let xl1 = self.dim_range[0].0;
        let xh1 = self.dim_range[0].1;
        let yl1 = self.dim_range[1].0;
        let yh1 = self.dim_range[1].1;
        let zl1 = self.dim_range[1].0;
        let zh1 = self.dim_range[1].1;

        let xl2 = other.dim_range[0].0;
        let xh2 = other.dim_range[0].1;
        let yl2 = other.dim_range[1].0;
        let yh2 = other.dim_range[1].1;
        let zl2 = other.dim_range[1].0;
        let zh2 = other.dim_range[1].1;

        let overlap_x = (xh1 >= xl2) && (xl1 <= xh2);
        let overlap_y = (yh1 >= yl2) && (yl1 <= yh2);
        let overlap_z = (zh1 >= zl2) && (zl1 <= zh2);

        overlap_x && overlap_y && overlap_z
    }

    fn volume(&self) -> u64 {
        let xl = (self.dim_range[0].1 - self.dim_range[0].0 + 1) as u64;
        let yl = (self.dim_range[1].1 - self.dim_range[1].0 + 1) as u64;
        let zl = (self.dim_range[2].1 - self.dim_range[2].0 + 1) as u64;

        xl * yl * zl
    }
}

fn push_check(vec: &mut Vec<Block>, block: Block) {
    if block.dim_range[0].0 > block.dim_range[0].1
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

fn block_slice(victim: Block, block: Block) -> Vec<Block> {
    if victim.overlaps(block) {
        let a = dim_slice(victim, 0, block.dim_range[0]);

        let mut b = Vec::new();
        for scan in a {
            b.extend(dim_slice(scan, 1, block.dim_range[1]));
        }

        let mut c = Vec::new();
        for scan in b {
            c.extend(dim_slice(scan, 2, block.dim_range[2]));
        }

        c
    } else {
        vec![victim]
    }
}

fn volume(state: &Vec<Block>) -> u64 {
    state.iter().map(|block| block.volume()).sum::<u64>()
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
                } else {
                    push_check(&mut filtered, elem);
                }
            }
            state = filtered;
            if on_off == "on" {
                push_check(&mut state, block);
            }
        }
    }

    println!("{}", volume(&state));
}
