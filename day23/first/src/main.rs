use std::io::{self, BufRead};

struct Board {
    tiles: Vec<Vec<char>>,
}

fn destination_col(amphipods_type: char) -> usize {
    match amphipods_type {
        'A' => 3,
        'B' => 5,
        'C' => 7,
        'D' => 9,
        _ => panic!(),
    }
}

impl Board {
    fn show(&self) {
        for line in self.tiles.clone() {
            for elem in line {
                print!("{elem}");
            }
            println!();
        }
    }
    fn get_ordered_amphipods(&self) -> Vec<(usize, usize, char)> {
        let mut amphipods_position: Vec<(usize, usize, char)> = Vec::new();

        for amphipod_type in ['A', 'B', 'C', 'D'] {
            for (row, line) in self.tiles.clone().into_iter().enumerate() {
                for (col, elem) in line.into_iter().enumerate() {
                    if elem == amphipod_type {
                        amphipods_position.push((row, col, elem));
                    }
                }
            }
        }
        amphipods_position
    }
    fn move_one(&self, row: usize, col: usize) -> Vec<(usize, usize, usize)> {
        let mut ret: Vec<(usize, usize, usize)> = Vec::new();

        if row < 2 {
            println!("{} {}", file!(), line!());
            return ret;
        } else {
            let amphipod_type = self.tiles[row][col];
            if col == destination_col(amphipod_type) {
                let mut scan = row + 1;
                let mut complete = true;
                while scan < self.tiles.len() - 1 {
                    if self.tiles[scan][col] != amphipod_type {
                        complete = false;
                    }
                    scan += 1;
                }
                if complete {
                    println!("{} {}", file!(), line!());
                    return ret;
                }
            }

            let mut steps = 0;
            let mut scan = row - 1;
            while scan > 1 {
                if self.tiles[scan][col] != '.' {
                    println!("{} {}", file!(), line!());
                    return ret;
                }
                scan -= 1;
                steps += 1;
            }

            let mut accumulate: Vec<(usize, usize, usize)> = Vec::new();
            accumulate.push((scan, col, steps));

            let mut left = col;
            let mut left_steps = steps;
            let mut right = col;
            let mut right_steps = steps;

            loop {
                let mut moved = false;
                if self.tiles[scan][left - 1] == '.' {
                    left = left - 1;
                    left_steps += 1;
                    moved = true;
                    accumulate.push((scan, left, left_steps));
                }

                if self.tiles[scan][right + 1] == '.' {
                    right = right + 1;
                    right_steps += 1;
                    moved = true;
                    accumulate.push((scan, right, right_steps));
                }

                if !moved {
                    break;
                }
            }

            for elem in accumulate {
                match elem {
                    (1, 3, _) | (1, 5, _) | (1, 7, _) | (1, 9, _) => {}
                    _ => {
                        ret.push(elem);
                    }
                }
            }
        }
        println!("{} {}", file!(), line!());
        ret
    }
    fn move_two(self, row: usize, col: usize) -> Vec<(usize, usize, usize)> {
        let mut ret: Vec<(usize, usize, usize)> = Vec::new();
        let amphipod_type = self.tiles[row][col];

        let dest_col = destination_col(amphipod_type);

        let mut state = '.';
        let mut space_row: Option<usize> = None;

        for row in 2..self.tiles.len() - 1 {
            let tile = self.tiles[row][dest_col];

            if state == '.' && tile == '.' {
                space_row = Some(row);
            } else if state == '.' && tile == amphipod_type {
                state = tile;
            } else if state == amphipod_type && tile == amphipod_type {
            } else {
                space_row = None;
                break;
            }
        }
        if let Some(row) = space_row {
            assert!(dest_col != col);
            let go_left = dest_col < col;
            let mut scan = col;
            let mut steps = 0;

            loop {
                scan = if go_left { scan - 1 } else { scan + 1 };

                if self.tiles[row][dest_col] != '.' {
                    return ret;
                }

                steps += 1;

                if scan == dest_col {
                    break;
                }
            }
            ret.push((row, dest_col, steps + row - 1));
        }
        ret
    }
}

fn main() {
    let mut board = Board { tiles: Vec::new() };
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        board.tiles.push(line.chars().collect());
    }
    board.show();
    println!("{:?}", board.move_two(1, 1));
}
