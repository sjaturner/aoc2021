use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut last: Option<i32> = None;
    let mut increased = 0;
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let depth: i32 = line.parse().unwrap();

        match last {
            Some(val) => {
                if depth > val {
                    increased += 1;
                }
            }
            _ => (),
        }
        last = Some(depth);
    }
    println!("{increased}")
}
