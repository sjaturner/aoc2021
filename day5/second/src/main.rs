use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut overlaps: HashMap<(i32, i32), u32> = HashMap::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let elems = line.split_whitespace().collect::<Vec<&str>>();
        assert!(elems[1] == "->");
        let parser = |s: &str| {
            s.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        };
        let a = parser(elems[0]);
        let b = parser(elems[2]);
        //      println!("{} {} {} {}", a[0], a[1], b[0], b[1]);
        let x1 = a[0];
        let y1 = a[1];
        let x2 = b[0];
        let y2 = b[1];

        if x1 == x2 {
            // Vertical line
            for y in min(y1, y2)..=max(y1, y2) {
                *overlaps.entry((x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            // Horizontal line
            for x in min(x1, x2)..=max(x1, x2) {
                *overlaps.entry((x, y1)).or_insert(0) += 1;
            }
        } else {
            assert!(i32::abs(x1 - x2) == i32::abs(y1 - y2));
            let dx = if x1 < x2 { 1 } else { -1 };
            let dy = if y1 < y2 { 1 } else { -1 };

            for step in 0..=i32::abs(x1 - x2) {
                *overlaps
                    .entry((x1 + step * dx, y1 + step * dy))
                    .or_insert(0) += 1;
            }
        }
    }

    println!(
        "{}",
        overlaps
            .into_iter()
            .map(|key_value| if key_value.1 > 1 { 1 } else { 0 })
            .sum::<u32>()
    );
}
