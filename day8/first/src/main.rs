use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut total = 0;
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let segs: Vec<&str> = line
            .split_whitespace()
            .filter(|&elem| elem != "|")
            .collect();

        for index in segs.len() - 4..segs.len() {
            if false {
                print!("{:?}", segs[index]);
            }
            total += match segs[index].len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            };
        }
    }
    println!("{total}");
}
