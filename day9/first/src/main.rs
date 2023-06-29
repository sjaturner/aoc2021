use std::io::{self, BufRead};

fn get_cell(row: i32, col: i32) -> Option<i32> {
    None
}

fn main() {
    let stdin = io::stdin();
    let mut array: Vec<Vec<i32>> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        array.push(
            line.chars()
                .map(|digit| (digit as i32 - '0' as i32))
                .collect(),
        );
    }

    let rows = array.len();
    let cols = array[0].len();
}
