use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut scanners: Vec<Vec<(i32, i32, i32)>> = Vec::new();
    let mut build = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let line = line.trim();

        if line.len() == 0 {
        } else if line.len() >= 3 && line[0..3] == "---".to_string() {
            if build.len() != 0 {
                scanners.push(build.to_vec());
                build.clear();
            }
            let scanner_number = line.split(' ').collect::<Vec<&str>>()[2]
                .parse::<usize>()
                .unwrap();
        } else {
            let vals: Vec<i32> = line
                .split(',')
                .map(|chunk| chunk.parse::<i32>().unwrap())
                .collect();
            build.push((vals[0], vals[1], vals[2]));
        }
    }
    scanners.push(build.to_vec());
    build.clear();

    println!("{:?}", scanners);
}
