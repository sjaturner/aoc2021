use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut ones = Vec::new();
    let mut zero = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let line = line.trim();
        let length = line.len();

        if ones.len() == 0 {
            for _ in 0..length {
                ones.push(0);
                zero.push(0);
            }
        } else if ones.len() != length {
            assert!(false);
        }

        for (index, character) in line.chars().rev().enumerate() {
            if character == '1' {
                ones[index] += 1;
            } else {
                zero[index] += 1;
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for index in 0..ones.len() {
        if ones[index] > zero[index] {
            gamma |= 1 << index;
        } else if ones[index] < zero[index] {
            epsilon |= 1 << index;
        } else {
            assert!(false);
        }
    }

    println!("{gamma} {epsilon} {}", gamma * epsilon);
}
