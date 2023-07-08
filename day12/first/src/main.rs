use std::collections::HashMap;
use std::io::{self, BufRead};

fn add_lut(lut: &mut Vec<(String, bool, bool)>, item: &str) -> (u32, bool) {
    let caps = match item.chars().nth(0).unwrap() {
        'A'..='Z' => true,
        _ => false,
    };
    let start = item == "start";
    let end = item == "end";
    if let Some(index) = lut.iter().position(|r| r.0 == item) {
        (index as u32, start)
    } else {
        lut.push((item.to_string(), caps, end));
        (lut.len() as u32 - 1, start)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut network: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut lut: Vec<(String, bool, bool)> = Vec::new();
    let mut start_index: Option<u32> = None;

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let nodes: Vec<&str> = line.split("-").collect();
        let a = add_lut(&mut lut, nodes[0]);
        let b = add_lut(&mut lut, nodes[1]);

        if a.1 {
            start_index = Some(a.0);
        }

        network.entry(a.0).or_insert(Vec::new()).push(b.0);
        network.entry(b.0).or_insert(Vec::new()).push(a.0);
    }
    println!("{:?}", network);
    for index in 0..lut.len() {
        println!("{:?}", lut[index])
    }
    println!("{:?}", start_index);
}
