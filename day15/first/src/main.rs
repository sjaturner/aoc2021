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
    let cave = Cave::load();

    println!("{:?}", cave);
}
