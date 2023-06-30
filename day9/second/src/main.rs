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

fn basin(array: &Vec<Vec<i32>>, row: i32, col: i32) -> u32 {
    let deltas = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut basin_cells: Vec<(i32, i32)> = Vec::new();
    basin_cells.push((row, col));
    let mut top: usize = 0;

    loop {
        let basin_cells_len = basin_cells.len();
        let mut added = false;
        for index in top..basin_cells_len {
            let (row_test, col_test) = basin_cells[index];
            if let Some(center) = get_cell(array, row_test, col_test) {
                if false {
                    println!("center {:?}", center);
                }
                for delta in deltas {
                    let row = row_test + delta.0;
                    let col = col_test + delta.1;
                    if let Some(peripheral) = get_cell(array, row, col) {
                        if false {
                            println!("    peripheral {:?} {row} {col}", peripheral);
                        }
                        if peripheral != 9 && peripheral > center {
                            basin_cells.push((row, col));
                            added = true;
                        }
                    }
                }
            }
        }
        if false {
            println!();
        }
        if !added {
            break;
        }
        top = basin_cells_len;
    }
    basin_cells.sort();
    basin_cells.dedup();
    basin_cells.len() as u32
}

fn is_low_point(array: &Vec<Vec<i32>>, row: i32, col: i32) -> Option<i32> {
    let deltas = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let current_cell = get_cell(array, row, col).unwrap();
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
    let mut results: Vec<u32> = Vec::new();

    for col in 0..cols {
        for row in 0..rows {
            if is_low_point(&array, row, col) != None {
                results.push(basin(&array, row, col));
            }
        }
    }
    results.sort();
    results.reverse();
    results.truncate(3);
    let product = results.iter().product::<u32>();
    println!("{:?}", product);
}
