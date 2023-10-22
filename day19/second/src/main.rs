use std::cmp::max;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let elems = line.split_whitespace().collect::<Vec<&str>>();
        let elems: Vec<i32> = elems
            .iter()
            .map(|item| item.parse::<i32>().unwrap())
            .collect();
        input.push(elems);
    }
    let mut max_manhatten = 0;
    for outer in 0..input.len() {
        for inner in 0..input.len() {
            let dx = (input[outer][0] - input[inner][0]).abs();
            let dy = (input[outer][1] - input[inner][1]).abs();
            let dz = (input[outer][2] - input[inner][2]).abs();

            max_manhatten = max(max_manhatten, dx + dy + dz);
        }
    }
    println!("{}", max_manhatten);
}
