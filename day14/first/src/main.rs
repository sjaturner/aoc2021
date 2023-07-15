use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Pair {
    chars: (char, char),
    insert: Option<char>,
}

fn main() {
    let stdin = io::stdin();
    let mut seed: Vec<char> = Vec::new();
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
            let line: Vec<&str> = line.split_whitespace().collect();
            let pattern: Vec<char> = line[0].chars().collect();
            let insert: Vec<char> = line[2].chars().collect();
            lut.insert((pattern[0], pattern[1]), insert[0]);
        }
    }

    for _ in 0..10 {
        let mut pairs: Vec<Pair> = Vec::new();

        for index in 0..seed.len() - 1 {
            pairs.push(Pair {
                chars: (seed[index], seed[index + 1]),
                insert: None,
            })
        }

        for pair in pairs.iter_mut() {
            if lut.contains_key(&pair.chars) {
                if let Some(insertion) = lut.get(&pair.chars) {
                    pair.insert = Some(*insertion);
                }
            }
        }

        let mut next: Vec<char> = Vec::new();
        for pair in &pairs {
            next.push(pair.chars.0);
            if let Some(insert) = pair.insert {
                next.push(insert);
            }
        }

        next.push(pairs.last().unwrap().chars.1);
        seed = next.clone();
    }
    println!("{:?}", seed.len());
    let mut collate: HashMap<char, u64> = HashMap::new();
    for character in seed {
        *collate.entry(character).or_insert(0) += 1;
    }
    println!("{:?}", collate);
}
