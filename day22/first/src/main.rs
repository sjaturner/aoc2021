use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut on: HashSet<(i32, i32, i32)> = HashSet::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let line = line.as_str();

        let r = Regex::new(r"(off|on) x=([-+0-9]*)\.\.([-+0-9]*),y=([-+0-9]*)\.\.([-+0-9]*),z=([-+0-9]*)\.\.([-+0-9]*)").unwrap();
        let caps = r.captures(line).unwrap();
        let on_off = &caps[1];
        let xl = cmp::max(caps[2].parse::<i32>().unwrap(), -50);
        let xu = cmp::min(caps[3].parse::<i32>().unwrap(), 50);
        let yl = cmp::max(caps[4].parse::<i32>().unwrap(), -50);
        let yu = cmp::min(caps[5].parse::<i32>().unwrap(), 50);
        let zl = cmp::max(caps[6].parse::<i32>().unwrap(), -50);
        let zu = cmp::min(caps[7].parse::<i32>().unwrap(), 50);

        for x in xl..=xu {
            for y in yl..=yu {
                for z in zl..=zu {
                    if on_off == "on" {
                        on.insert((x, y, z));
                    } else {
                        if on.contains(&(x, y, z)) {
                            on.remove(&(x, y, z));
                        }
                    }
                }
            }
        }
    }
    println!("{}", on.len());
}
