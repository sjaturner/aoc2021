use itertools::Itertools;
use std::io::{self, BufRead};

const VALID: &'static [&'static str] = &[
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn try_map(segs: &Vec<&str>, lut: Vec<&char>) -> Option<[u32; 4]> {
    let mut matches = Vec::new();
    let segs_len = segs.len();
    for elem in segs {
        let mut mapped: Vec<char> = Vec::new();
        for segment_letter in elem.chars() {
            let map_index = (segment_letter as u32 - 'a' as u32) as usize;
            let map_char = lut[map_index];
            mapped.push(*map_char);
        }
        mapped.sort();
        let mapped: String = mapped.iter().collect();
        if false {
            println!("{:?}", mapped);
        }
        match VALID.iter().position(|&r| r == mapped) {
            None => return None,
            Some(digit) => matches.push(digit as u32),
        };
    }
    if matches.len() != segs_len {
        return None;
    }
    while matches.len() > 4 {
        matches.remove(0);
    }
    return Some([matches[0], matches[1], matches[2], matches[3]]);
}

fn main() {
    let stdin = io::stdin();
    let mut total = 0;
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let segs: Vec<&str> = line
            .split_whitespace()
            .filter(|&elem| elem != "|")
            .collect();

        let items = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        for map in items.iter().permutations(items.len()) {
            if false {
                println!("{:?}", map);
            }
            match try_map(&segs, map) {
                None => {}
                Some(result) => {
                    if false {
                        println!("{:?}", result);
                    }
                    let mut value: u32 = 0;
                    for digit in result {
                        value *= 10;
                        value += digit;
                    }
                    total += value;
                    break;
                }
            }
        }
    }
    println!("{total}");
}
