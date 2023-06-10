use std::io::{self, BufRead};
use std::iter;

fn dump(lines: &Vec<String>, keep: &Vec<i32>) {
    for index in 0..lines.len() {
        println!("{} {}", keep[index], lines[index]);
    }
}

fn filter(lines: &Vec<String>, keep: &mut Vec<i32>, offset: usize, sel: u32) -> usize {
    let mut tally: [u32; 2] = [0, 0];

    for index in 0..lines.len() {
        if keep[index] != 0 {
            let character = lines[index].chars().nth(offset).unwrap();
            //          println!("{character}");
            if character == '1' {
                tally[1] += 1;
            } else if character == '0' {
                tally[0] += 1;
            } else {
                assert!(false);
            }
        }
    }

    //  println!("{:?}", tally);

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

    for index in 0..lines.len() {
        let character = lines[index].chars().nth(offset).unwrap();
        if character != retain {
            keep[index] = 0;
        }
    }

    return keep.iter().sum::<i32>() as usize;
}

fn metric(lines: &Vec<String>, sel: u32) -> usize {
    let mut keep: Vec<i32> = iter::repeat(1).take(lines.len()).collect();
    let mut offset = 0;

    while filter(&lines, &mut keep, offset, sel) > 1 {
        offset += 1;
    }

    if let Some(index) = keep.iter().position(|&x| x != 0) {
        usize::from_str_radix(&lines[index], 2).unwrap()
    } else {
        0
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let line = line.trim();
        lines.push(String::from(line))
    }
    let mut offset;

    let mut keep: Vec<i32> = iter::repeat(1).take(lines.len()).collect();
    offset = 0;
    while filter(&lines, &mut keep, offset, 1) > 1 {
        //      dump(&lines, &keep);
        offset += 1;
    }

    let oxygen = metric(&lines, 1);
    let carbon_dioxide = metric(&lines, 0);

    println!("{} {} {}", oxygen, carbon_dioxide, oxygen * carbon_dioxide);
}
