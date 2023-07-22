use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};
use std::process::exit;

#[derive(Debug)]
struct Pair {
    chars: (char, char),
    insert: Option<char>,
}

fn recurse(lut: &HashMap<(char, char), char>, recur: i32, l: char, r: char) {
    if let Some(insertion) = lut.get(&(l, r)) {
        let m = *insertion;

        if recur > 0 {
            recurse(lut, recur - 1, l, m);
            recurse(lut, recur - 1, m, r);
        } else {
            print!("{} ", l);
            print!("{} ", m);
        }
    } else {
        assert!(true);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut seed: Vec<char> = Vec::new();
    let mut lut: HashMap<(char, char), char> = HashMap::new();

    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        exit(0);
    }

    let depth = args[1].parse::<i32>().unwrap();

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
            let line: Vec<&str> = line.split_whitespace().collect();
            let pattern: Vec<char> = line[0].chars().collect();
            let insert: Vec<char> = line[2].chars().collect();
            lut.insert((pattern[0], pattern[1]), insert[0]);
        }
    }

    if depth > 0 {
        recurse(&lut, depth - 1, 'N', 'N');
    }
    println!();
}
