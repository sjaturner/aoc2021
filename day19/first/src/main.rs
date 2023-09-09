use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

#[derive(Debug)]
struct Scanner {
    a: Vec<i32>,
    b: Vec<i32>,
    c: Vec<i32>,
    mag: HashMap<i32, Vec<(usize, usize)>>,
}

fn main() {
    let stdin = io::stdin();
    let mut scanners: HashMap<u32, Scanner> = HashMap::new();
    let mut scanner_number: u32 = 0;

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let line = line.trim();

        if line.len() == 0 {
        } else if line.len() >= 3 && line[0..3] == "---".to_string() {
            scanner_number = line.split(' ').collect::<Vec<&str>>()[2]
                .parse::<u32>()
                .unwrap();
            scanners.insert(
                scanner_number,
                Scanner {
                    a: Vec::new(),
                    b: Vec::new(),
                    c: Vec::new(),
                    mag: HashMap::new(),
                },
            );
        } else {
            let vals: Vec<i32> = line
                .split(',')
                .map(|chunk| chunk.parse::<i32>().unwrap())
                .collect();
            let entry = scanners.get_mut(&scanner_number).unwrap();
            entry.a.push(vals[0]);
            entry.b.push(vals[1]);
            entry.c.push(vals[2]);
        }
    }

    let keys: Vec<u32> = scanners.keys().map(|x| *x).collect();
    for key in keys {
        let entry = scanners.get_mut(&key).unwrap();

        for outer in 0..entry.a.len() {
            for inner in 0..outer {
                let da = entry.a[outer] - entry.a[inner];
                let db = entry.b[outer] - entry.b[inner];
                let dc = entry.c[outer] - entry.c[inner];
                let mag = da * da + db + db + dc * dc;

                entry
                    .mag
                    .entry(mag)
                    .or_insert(Vec::new())
                    .push((outer, inner));
            }
        }
    }
    let keys: Vec<u32> = scanners.keys().map(|x| *x).collect();
    let max_key = *keys.iter().max().unwrap();
    assert!(keys.len() == max_key as usize + 1);

    for outer in 0..=max_key {
        for inner in outer + 1..=max_key {
            let inner_set: HashSet<i32> =
                scanners.get(&inner).unwrap().mag.keys().copied().collect();
            let outer_set: HashSet<i32> =
                scanners.get(&outer).unwrap().mag.keys().copied().collect();
            let intersection = inner_set.intersection(&outer_set);
            println!("{outer} {inner} {:?}", intersection);
            for item in intersection {
                println!("item: {item}");
                println!(
                    "outer: {outer}: {:?}",
                    scanners.get(&outer).unwrap().mag.get(item)
                );
                println!(
                    "inner: {inner}: {:?}",
                    scanners.get(&inner).unwrap().mag.get(item)
                );
            }
        }
    }

    if false {
        println!("{:?}", scanners);
    }
}
