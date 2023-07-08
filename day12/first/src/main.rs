use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut network: HashMap<String, Vec<String>> = HashMap::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let nodes: Vec<&str> = line.split("-").collect();
        let a = nodes[0].to_string();
        let b = nodes[1].to_string();
        network.entry(a).or_insert(Vec::new()).push(b);
    }
    println!("{:?}", network.get("A").unwrap());
}
