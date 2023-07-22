use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};
use std::process::exit;

#[derive(Copy, Clone)]
struct Tally {
    letters: [u64; LETTERS],
}

impl Default for Tally {
    fn default() -> Tally {
        Tally {
            letters: [0; LETTERS],
        }
    }
}

impl Tally {
    fn inc(&mut self, c: char) {
        self.letters[c as usize] += 1;
    }
}

const LETTERS: usize = 'Z' as usize - 'A' as usize;
const DEPTH: usize = 20;

fn recurse(
    lut: &HashMap<(char, char), char>,
    recur: i32,
    memo: &mut [[[Tally; LETTERS]; LETTERS]; DEPTH],
    l: char,
    r: char,
) {
    if let Some(insertion) = lut.get(&(l, r)) {
        let m = *insertion;

        if recur > 0 {
            recurse(lut, recur - 1, memo, l, m);
            recurse(lut, recur - 1, memo, m, r);
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
    let mut memo: [[[Tally; LETTERS]; LETTERS]; DEPTH] =
        [[[Tally::default(); LETTERS]; LETTERS]; DEPTH];

    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        exit(0);
    }

    let depth = args[1].parse::<i32>().unwrap();

    if depth as usize >= DEPTH {
        exit(0);
    }

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
        recurse(&lut, depth - 1, &mut memo, 'N', 'N');
    }
    println!();
}
