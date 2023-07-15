use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut seed: Vec<char>;
    let mut lut: HashMap<(char, char), char> = HashMap::new();

    for (index, line) in stdin
        .lock()
        .lines()
        .filter(|line| !line.as_ref().unwrap().is_empty())
        .enumerate()
    {
        let line = line.expect("Could not read line from standard in");
        let line = line.trim();

        if index == 0 {
            seed = line.chars().collect();
            println!("{:?}", seed);
        } else {
            let line:Vec<&str> = line.split_whitespace().collect();
            let pattern = line[0];
            let insert = line[2];
            println!("{pattern} {insert}");
        }
    }
}
