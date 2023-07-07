use std::fmt;
use std::io::{self, BufRead};

#[derive(Clone)]
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
    fn add(&mut self, vals: &Cave, count: &mut u32) -> Option<Cave> {
        let mut ret = self.blank(0);
        let mut flashes = false;
        for row in 0..self.rows {
            for col in 0..self.cols {
                let before = self.get(row, col);
                let after = before + vals.get(row, col);

                if before < 10 && after >= 10 {
                    flashes = true;
                    *count += 1;

                    for delta_row in -1..=1 {
                        for delta_col in -1..=1 {
                            if delta_row == 0 && delta_col == 0 {
                                continue;
                            }

                            let scan_row = row + delta_row;
                            let scan_col = col + delta_col;
                            if scan_row >= 0 && scan_row < self.rows {
                                if scan_col >= 0 && scan_col < self.cols {
                                    ret.set(scan_row, scan_col, ret.get(scan_row, scan_col) + 1);
                                }
                            }
                        }
                    }
                }
                self.set(row, col, after);
            }
        }
        if flashes {
            Some(ret)
        } else {
            None
        }
    }
    fn reset(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.get(row, col) >= 10 {
                    self.set(row, col, 0);
                }
            }
        }
    }
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let _ = write!(f, "{}", self.get(row, col));
            }
            println!()
        }
        Ok(())
    }
}

fn main() {
    let mut cave = Cave::load();

    let goes = 100;
    let mut count = 0;

    for _ in 0..goes {
        let mut pump = cave.blank(1);

        loop {
            if let Some(foo) = cave.add(&pump, &mut count) {
                pump = foo;
                if false {
                    println!("{:?}", pump);
                }
            } else {
                break;
            }
        }
        cave.reset();
        println!("{:?}", cave);
    }
    println!("{count}");
}
