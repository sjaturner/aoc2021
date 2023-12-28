use std::fmt;
use std::io::{self, BufRead};

#[derive(Clone)]
struct Cave {
    rows: usize,
    cols: usize,
    cells: Vec<char>,
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
            for value in line.chars() {
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
    fn modulo(a: i32, b: usize) -> usize {
        let b = b as i32;
        (((a % b) + b) % b) as usize
    }
    fn get(&self, row: i32, col: i32) -> char {
        let row = Self::modulo(row, self.rows);
        let col = Self::modulo(col, self.cols);
        self.cells[(row * self.cols) + col]
    }
    fn set(&mut self, row: i32, col: i32, val: char) {
        let row = Self::modulo(row, self.rows);
        let col = Self::modulo(col, self.cols);
        self.cells[(row * self.cols) + col] = val;
    }
    fn step(&mut self) -> u32 {
        let mut count = 0;
        let mut moveable = Vec::new();
        for row in 0..self.rows as i32 {
            for col in 0..self.cols as i32 {
                if self.get(row, col) == '>' && self.get(row, col + 1) == '.' {
                    moveable.push((row, col));
                    count += 1;
                }
            }
        }
        for (row, col) in moveable {
            self.set(row, col, '.');
            self.set(row, col + 1, '>');
        }
        let mut moveable = Vec::new();
        for col in 0..self.cols as i32 {
            for row in 0..self.rows as i32 {
                if self.get(row, col) == 'v' && self.get(row + 1, col) == '.' {
                    moveable.push((row, col));
                    count += 1;
                }
            }
        }
        for (row, col) in moveable {
            self.set(row, col, '.');
            self.set(row + 1, col, 'v');
        }
        count
    }
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let _ = write!(f, "{}", self.get(row as i32, col as i32));
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn main() {
    let mut cave = Cave::load();
    let mut step = 1;

    loop {
        println!("{step}");
        println!("{:?}", cave);
        let mut count = cave.step();
        println!("{count}");
        if count == 0 {
            println!(">> {:?}", step);
            break;
        }
        step += 1;
    }
}
