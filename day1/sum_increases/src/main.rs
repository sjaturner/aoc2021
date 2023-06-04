use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let depth: i32 = line.parse().unwrap();
        input.push(depth);
    }
    let mut sum = Vec::new();
    for index in 0..input.len() - 2 {
        let mut total = 0;
        for offset in 0..=2 {
            total += input[index + offset];
        }
        sum.push(total);
    }
//  println!("{:?}", sum);
    let mut increased = 0;
    for index in 0..sum.len() - 1 {
        if sum[index + 1] > sum[index] {
            increased += 1;
        }
    }
    println!("{increased}");
}
