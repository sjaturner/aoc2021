use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

#[derive(Debug)]
struct Scanner {
    a: Vec<i32>,
    b: Vec<i32>,
    c: Vec<i32>,
    mag: HashMap<i32, Vec<(usize, usize)>>,
}

#[derive(Debug)]
struct Mapper {
    swap: bool,
    xflip: i32,
    yflip: i32,
    xoff: i32,
    yoff: i32,
}

fn transform(mapper: &Mapper, p: (i32, i32)) -> (i32, i32) {
    let bx = if mapper.swap { p.1 } else { p.0 };
    let by = if mapper.swap { p.0 } else { p.1 };
    let bx = mapper.xflip * bx;
    let by = mapper.yflip * by;
    let bx = bx + mapper.xoff;
    let by = by + mapper.yoff;

    (bx, by)
}

fn score(mapper: Mapper, a: &Vec<(i32, i32)>, b: &Vec<(i32, i32)>) -> u32 {
    let mut total = 0;

    for index in 0..a.len() {
        let ax = a[index].0;
        let ay = a[index].1;
        let (bx, by) = transform(&mapper, (b[index].0, b[index].1));

        if ax == bx && ay == by {
            total += 1;
        }
    }
    total
}

fn best_fit(a: &Vec<(i32, i32)>, b: &Vec<(i32, i32)>) -> Mapper {
    assert!(a.len() == b.len());
    for swap in [false, true] {
        for xflip in [-1, 1] {
            for yflip in [-1, 1] {
                let mut mapper = Mapper {
                    xoff: 0,
                    yoff: 0,
                    swap,
                    xflip,
                    yflip,
                };
                let (xoff, yoff) = transform(&mapper, b[0]);
                mapper.xoff = -xoff;
                mapper.yoff = -yoff;
                println!("{:?}", mapper);
                score(mapper, &a, &b);
            }
        }
    }

    Mapper {
        swap: false,
        xflip: 0,
        yflip: 0,
        xoff: 0,
        yoff: 0,
    }
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
            let count = intersection.clone().count();

            // The instructions say that there are at least this many overlapping points.
            if count < 12 {
                continue;
            }

            println!("{outer} {inner} {:?} {}", intersection, count);
            let mut ol: Vec<(i32, i32, i32)> = Vec::new();
            let mut il: Vec<(i32, i32, i32)> = Vec::new();
            for item in intersection {
                if let Some(outer_list) = scanners.get(&outer).unwrap().mag.get(item) {
                    if outer_list.len() == 1 {
                        if let Some(inner_list) = scanners.get(&inner).unwrap().mag.get(item) {
                            if inner_list.len() == 1 {
                                let os = scanners.get(&outer).unwrap();
                                let oo = outer_list[0].0;
                                ol.push((os.a[oo], os.b[oo], os.c[oo]));

                                let is = scanners.get(&inner).unwrap();
                                let io = outer_list[0].0;
                                il.push((is.a[io], is.b[io], is.c[io]));
                            }
                        }
                    }
                }
            }
            println!("outer: {outer}: {:?}", ol);
            println!("inner: {inner}: {:?}", il);
        }
    }

    if false {
        println!("{:?}", scanners);
    }
}
