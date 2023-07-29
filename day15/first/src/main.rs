use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io::{self, BufRead};

#[derive(Clone)]
struct Cave {
    rows: i32,
    cols: i32,
    cells: Vec<i32>,
}

impl Cave {
    fn load() -> Cave {
        let stdin = io::stdin();
        let mut cave = Cave {
            rows: 0,
            cols: 0,
            cells: Vec::new(),
        };
        let mut rows = 0;
        for line in stdin.lock().lines() {
            let line = line.expect("Could not read line from standard in");

            let mut cols = 0;
            for value in line.chars().map(|digit| digit as i32 - '0' as i32) {
                cave.cells.push(value);
                cols += 1;
            }

            if cave.cols == 0 {
                cave.cols = cols;
            } else {
                assert!(cave.cols == cols);
            }

            rows += 1;
        }
        cave.rows = rows;
        cave
    }
    fn get(&self, row: i32, col: i32) -> i32 {
        self.cells[(row * self.cols) as usize + col as usize]
    }
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let _ = write!(f, "{}", self.get(row, col));
            }
            writeln!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Cell {
    risk: i32,
    risk_total: Option<i32>,
    from: Option<(i32, i32)>,
    neighbours: [Option<(i32, i32)>; 4],
}

fn connection(cave: &Cave, row: i32, col: i32) -> Option<(i32, i32)> {
    if row >= 0 && row < cave.rows && col >= 0 && col < cave.cols {
        Some((row, col))
    } else {
        None
    }
}

fn neighbours(cave: &Cave, row: i32, col: i32) -> [Option<(i32, i32)>; 4] {
    [
        connection(cave, row + 0, col + 1),
        connection(cave, row + 1, col + 0),
        connection(cave, row - 1, col + 0),
        connection(cave, row + 0, col - 1),
    ]
}

fn main() {
    let cave = Cave::load();
    let mut cells: HashMap<(i32, i32), Cell> = HashMap::new();

    for row in 0..cave.rows {
        for col in 0..cave.cols {
            cells.insert(
                (row, col),
                Cell {
                    risk: cave.get(row, col),
                    risk_total: None,
                    from: None,
                    neighbours: neighbours(&cave, row, col),
                },
            );
        }
    }

    let mut done: HashSet<(i32, i32)> = HashSet::new();
    let mut full: HashSet<(i32, i32)> = HashSet::new();

    done.insert((0, 0));

    println!("{:?}", cave);
    println!("{:?}", cells);

    let mut from: Option<(i32, i32)> = None;
    let mut best: Option<(i32, i32)> = None;
    let mut risk: Option<i32> = None;
    for cell_pos in done {
        if full.contains(&cell_pos) {
            continue;
        }

        let curr = cells.get(&cell_pos).unwrap();

        for neighbour in curr.neighbours {
            if let Some(neighbour_pos) = neighbour {
                if let Some(neighbour) = cells.get(&neighbour_pos) {
                    if let Some(risk_total) = curr.risk_total {
                        let neighbour_risk = risk_total + neighbour.risk;
                        if let Some(lowest_risk) = risk {
                            if neighbour_risk < lowest_risk {
                                risk = Some(neighbour_risk);
                                best = Some(neighbour_pos);
                                from = Some(cell_pos);
                            }
                        } else {
                            risk = Some(neighbour_risk);
                            best = Some(neighbour_pos);
                            from = Some(cell_pos);
                        }
                    }
                }
            }
        }
    }
}
