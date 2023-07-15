use regex::Regex;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn folder(dots: &HashSet<(i32, i32)>, xy: char, line: i32) -> HashSet<(i32, i32)> {
    let xlat = |x: i32, y: i32| if xy == 'x' { (x, y) } else { (y, x) };
    let mut ret: HashSet<(i32, i32)> = HashSet::new();

    for item in dots {
        let (x, y) = xlat(item.0, item.1);

        assert!(x != line);

        let fx = if x > line { line - (x - line) } else { x };

        ret.insert(xlat(fx, y));
    }
    ret
}

fn render(dots: &HashSet<(i32, i32)>) {
    let mut plot: Vec<(i32, i32)> = dots.iter().map(|&(x, y)| (x, y)).collect();
    plot.sort();
    println!("{:?}", plot);
}

fn main() {
    let stdin = io::stdin();
    let mut dots: HashSet<(i32, i32)> = HashSet::new();
    let re = Regex::new(r"^fold along\s*([xy])\s*=\s*([0-9]*)").unwrap();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let line = line.trim();
        if line.len() == 0 {
            // Ignore blank lines
        } else {
            let mut folded = false;
            for (_, [f1, f2]) in re.captures_iter(line).map(|caps| caps.extract()) {
                dots = folder(
                    &dots,
                    f1.chars().nth(0).unwrap(),
                    f2.parse::<i32>().unwrap(),
                );
                render(&dots);
                folded = true;
            }
            if !folded {
                let pos: Vec<i32> = line
                    .split(",")
                    .map(|xy| xy.parse::<i32>().unwrap())
                    .collect();
                dots.insert((pos[0], pos[1]));
            }
        }
    }

    println!("{:?}", dots);
}
