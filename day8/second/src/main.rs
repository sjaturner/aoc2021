use std::io::{self, BufRead};

const VALID: &'static [&'static str] = &[
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let segs: Vec<&str> = line
            .split_whitespace()
            .filter(|&elem| elem != "|")
            .collect();
    }
}
