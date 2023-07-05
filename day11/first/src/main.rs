use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Cave {
    rows: i32,
    cols: i32,
    cells: Vec<i32>,
}

impl Cave {
    fn load() -> Cave {
        let stdin = io::stdin();
        let mut cave = Cave {
            rows: 0,
            cols: 0,
            cells: Vec::new(),
        };
        let mut rows = 0;
        for line in stdin.lock().lines() {
            let line = line.expect("Could not read line from standard in");

            let mut cols = 0;
            for value in line.chars().map(|digit| digit as i32 - '0' as i32) {
                cave.cells.push(value);
                cols += 1;
            }

            if cave.cols == 0 {
                cave.cols = cols;
            } else {
                assert!(cave.cols == cols);
            }

            rows += 1;
        }
        cave.rows = rows;
        cave
    }
    fn get(&self, row: i32, col: i32) -> i32 {
        self.cells[(row * self.cols) as usize + col as usize]
    }
    fn set(&mut self, row: i32, col: i32, val: i32) {
        self.cells[(row * self.cols) as usize + col as usize] = val;
    }
    fn blank(&mut self, fill: i32) -> Cave {
        let mut ret = Cave {
            rows: self.rows,
            cols: self.cols,
            cells: Vec::new(),
        };
        for _ in 0..self.rows * self.cols {
            ret.cells.push(fill);
        }
        ret
    }
    fn add(&mut self, vals: &Cave) -> Cave {
        let mut ret = self.blank(0);
        for index in 0usize..(self.rows * self.cols) as usize {
            let before = self.cells[index];
            let after = self.cells[index] + vals.cells[index];

            if before < 10 && after >= 10 {
            }
        }
        ret
    }
}
fn main() {
    let cave = Cave::load();

    println!("{:?}", cave);
    let mut new = cave.clone();
}
