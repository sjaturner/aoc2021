use std::collections::HashMap;
use std::io::{self, BufRead};

fn add_lut(lut: &mut Vec<(String, bool, bool, bool)>, item: &str) -> (u32, bool) {
    let caps = item.chars().next().unwrap().is_ascii_uppercase();
    let start = item == "start";
    let end = item == "end";
    if let Some(index) = lut.iter().position(|r| r.0 == item) {
        (index as u32, start)
    } else {
        lut.push((item.to_string(), caps, end, start));
        (lut.len() as u32 - 1, start)
    }
}

fn recurse(
    network: &HashMap<u32, Vec<u32>>,
    lut: &Vec<(String, bool, bool, bool)>,
    stack: &mut Vec<u32>,
    node_index: u32,
    permit: bool,
) {
    stack.push(node_index);
    let top_node = stack[stack.len() - 1];
    if let Some(list) = network.get(&top_node) {
        for next_node in list {
            let (_, can_revisit, end, start) = lut[*next_node as usize];

            if start {
            } else if end {
                for index in 0..stack.len() {
                    print!("{},", lut[stack[index] as usize].0)
                }
                println!("end");
            } else if can_revisit {
                recurse(network, lut, stack, *next_node, permit);
            } else {
                let mut already_visited = false;
                for index in 0..stack.len() {
                    if stack[index] == *next_node {
                        already_visited = true;
                        break;
                    }
                }
                if already_visited {
                    if permit {
                        recurse(network, lut, stack, *next_node, false);
                    }
                } else {
                    recurse(network, lut, stack, *next_node, permit);
                }
            }
        }
    }
    stack.pop();
}

fn main() {
    let stdin = io::stdin();
    let mut network: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut lut: Vec<(String, bool, bool, bool)> = Vec::new();
    let mut start_index: Option<u32> = None;

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let nodes: Vec<&str> = line.split('-').collect();
        let a = add_lut(&mut lut, nodes[0]);
        let b = add_lut(&mut lut, nodes[1]);

        if a.1 {
            start_index = Some(a.0);
        }

        network.entry(a.0).or_insert(Vec::new()).push(b.0);
        network.entry(b.0).or_insert(Vec::new()).push(a.0);
    }
    if false {
        println!("{:?}", network);
        for entry in &lut {
            println!("{:?}", entry);
        }
        println!("{:?}", start_index);
    }
    if let Some(index) = start_index {
        let mut stack: Vec<u32> = Vec::new();
        recurse(&network, &lut, &mut stack, index, true);
    }
}
