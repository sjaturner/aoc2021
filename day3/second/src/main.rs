use std::io::{self, BufRead};
use std::iter;

fn filter(lines: &Vec<String>, keep: &mut Vec<i32>, offset: u32, sel: u32) -> usize {
    return 0;
}

fn main() {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let line = line.trim();
        lines.push(String::from(line))
    }
    let mut keep: Vec<i32> = iter::repeat(0).take(lines.len()).collect();

    while filter(&lines, &mut keep, 0, 0) > 1 {
        todo!();
    }
}
