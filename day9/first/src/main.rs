use std::io::{self, BufRead};

fn get_cell(array: &Vec<Vec<i32>>, row: i32, col: i32) -> Option<i32> {
    let rows = array.len() as i32;
    let cols = array[0].len() as i32;

    if row >= rows || row < 0 || col >= cols || col < 0 {
        None
    } else {
        Some(array[row as usize][col as usize])
    }
}

fn is_low_point(array: &Vec<Vec<i32>>, row: i32, col: i32) -> Option<i32> {
    let current_cell = get_cell(array, row, col).unwrap();
    let deltas = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut matches = 0;
    let mut possible = 0;

    for delta in deltas {
        if let Some(value) = get_cell(array, row + delta.0, col + delta.1) {
            possible += 1;
            if value <= current_cell {
                return None;
            } else {
                matches += 1;
            }
        }
    }
    if matches == possible {
        Some(current_cell)
    } else {
        None
    }
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
    let rows = array.len() as i32;
    let cols = array[0].len() as i32;
    let mut risk_level_accumulation = 0;

    for col in 0..cols {
        for row in 0..rows {
            if let Some(value) = is_low_point(&array, row, col) {
                risk_level_accumulation += value + 1;
            }
        }
    }
    println!("{risk_level_accumulation}");
}
