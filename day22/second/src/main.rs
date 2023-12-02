use regex::Regex;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Block {
    dim_range: [(i32, i32); 3],
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

    fn fully_contains(&self, block: Block) -> bool {
        block.dim_range[0].0 >= self.dim_range[0].0
            && block.dim_range[0].1 <= self.dim_range[0].1
            && block.dim_range[1].0 >= self.dim_range[1].0
            && block.dim_range[1].1 <= self.dim_range[1].1
            && block.dim_range[2].0 >= self.dim_range[2].0
            && block.dim_range[2].1 <= self.dim_range[2].1
    }

    fn dim_slice(&self, dim: usize, range: (i32, i32)) -> Vec<Block> {
        let mut ret = Vec::new();

        let victim_dim_range = self.dim_range[dim];

        if victim_dim_range.1 < range.0
            || victim_dim_range.0 > range.1
            || victim_dim_range.0 > range.0 && victim_dim_range.1 < range.1
        {
            ret.push(*self);
        } else if victim_dim_range.0 < range.0 && victim_dim_range.1 > range.1 {
            let mut l = *self;

            l.dim_range[dim].1 = range.0 - 1;
            ret.push(l);

            let mut m = *self;
            m.dim_range[dim].0 = range.0;
            m.dim_range[dim].1 = range.1;
            ret.push(m);

            let mut r = *self;
            r.dim_range[dim].0 = range.1 + 1;
            ret.push(r);
        } else if victim_dim_range.1 > range.1 {
            let mut l = *self;
            l.dim_range[dim].1 = range.1;
            ret.push(l);

            let mut r = *self;
            r.dim_range[dim].0 = range.1 + 1;
            ret.push(r);
        } else if victim_dim_range.0 < range.0 {
            let mut l = *self;
            l.dim_range[dim].1 = range.0 - 1;
            ret.push(l);

            let mut r = *self;
            r.dim_range[dim].0 = range.0;
            ret.push(r);
        } else {
            ret.push(*self);
        }

        ret
    }
}

// If I were smart, I would use iterators more in the next two functions
fn block_slice(victim: Block, block: Block) -> Vec<Block> {
    if victim.overlaps(block) {
        let a = victim.dim_slice(0, block.dim_range[0]);

        let mut b = Vec::new();
        for scan in a {
            b.extend(scan.dim_slice(1, block.dim_range[1]));
        }

        let mut c = Vec::new();
        for scan in b {
            c.extend(scan.dim_slice(2, block.dim_range[2]));
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

    for scan in state {
        let sliced = block_slice(*scan, block);

        ret.extend(sliced);
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
            state.push(block);
        } else {
            state = slice(&state, block);
            let mut filtered = Vec::new();
            for elem in state {
                if block.fully_contains(elem) {
                } else {
                    filtered.push(elem);
                }
            }
            state = filtered;
            if on_off == "on" {
                state.push(block);
            }
        }
    }

    println!("{}", volume(&state));
}
