use std::collections::HashMap;
use std::io::{self, BufRead};

fn add_lut(lut: &mut Vec<String>, item: &str) -> u32 {
    if let Some(index) = lut.iter().position(|r| r == item) {
        index as u32
    } else {
        lut.push(item.to_string());
        lut.len() as u32 - 1
    }
}

fn main() {
    let stdin = io::stdin();
    let mut network: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut lut: Vec<String> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let nodes: Vec<&str> = line.split("-").collect();
        let a = add_lut(&mut lut, nodes[0]);
        let b = add_lut(&mut lut, nodes[1]);

        network.entry(a).or_insert(Vec::new()).push(b);
        network.entry(b).or_insert(Vec::new()).push(a);
    }
    println!("{:?}", network);
}
