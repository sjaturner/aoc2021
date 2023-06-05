use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut depth = 0;
    let mut forward = 0;
    let mut aim = 0;
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let line: Vec<&str> = line.split(' ').collect();
        let val = line[1].parse::<i32>().unwrap();

        match line[0] {
            "up" => aim -= val,
            "down" => aim += val,
            "forward" => {
                forward += val;
                depth += aim * val;
            }
            &_ => todo!(),
        }
    }
    println!("{} {} {}", depth, forward, depth * forward)
}
