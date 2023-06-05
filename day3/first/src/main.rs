use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut bits: Option<usize> = None;
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let line = line.trim();
        let length = line.len();

        if bits.is_none() {
            bits = Some(length);
        } else {
            assert!(bits == Some(length));
        }
    }
}
