use std::io::{self, BufRead};
use std::iter;

fn filter(lines: &Vec<String>, keep: &mut Vec<i32>, offset: usize, sel: u32) -> usize {
    let mut tally: [u32; 2] = [0, 0];

    for index in 0..lines.len() {
        if keep[index] != 0 {
            let character = lines[index].chars().nth(offset).unwrap();
            println!("{character}");
            if character == '1' {
                tally[1] += 1;
            } else if character == '0' {
                tally[0] += 1;
            } else {
                assert!(false);
            }
        }
    }

    println!("{:?}", tally);

    let retain = if sel == 1 {
        if tally[0] > tally[1] {
            '0'
        } else if tally[1] > tally[0] {
            '1'
        } else {
            '1'
        }
    } else {
        if tally[0] < tally[1] {
            '0'
        } else if tally[1] < tally[0] {
            '1'
        } else {
            '0'
        }
    };

    println!("{offset} {retain}");

    for index in 0..lines.len() {
        let character = lines[index].chars().nth(offset).unwrap();
        if character != retain {
            keep[index] = 0;
        }
    }

    return keep.iter().sum::<i32>() as usize;
}

fn main() {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let line = line.trim();
        lines.push(String::from(line))
    }
    let mut keep: Vec<i32> = iter::repeat(1).take(lines.len()).collect();

    let mut offset = 0;
    while filter(&lines, &mut keep, offset, 1) > 1 {
        for index in 0..keep.len() {
            if keep[index] != 0 {
                println!("{}", lines[index]);
            }
        }
        println!();
        offset += 1;
    }
}
