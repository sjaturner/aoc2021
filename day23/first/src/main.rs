use std::io::{self, BufRead};

#[derive(Clone)]
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

fn home(col: usize, amphipods_type: char) -> bool {
    amphipods_type != '.' && col == destination_col(amphipods_type)
}

fn move_cost(amphipods_type: char) -> u64 {
    match amphipods_type {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
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
    fn move_one(&self, row: usize, col: usize) -> Vec<(usize, usize, u64)> {
        let mut ret: Vec<(usize, usize, u64)> = Vec::new();

        if row < 2 {
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
                    return ret;
                }
            }

            let mut steps = 1;
            let mut scan = row - 1;
            while scan > 1 {
                if self.tiles[scan][col] != '.' {
                    return ret;
                }
                scan -= 1;
                steps += 1;
            }

            let mut accumulate: Vec<(usize, usize, u64)> = Vec::new();
            accumulate.push((scan, col, steps));

            let mut left = col;
            let mut left_steps = steps;
            let mut right = col;
            let mut right_steps = steps;

            loop {
                let mut moved = false;
                if self.tiles[scan][left - 1] == '.' {
                    left -= 1;
                    left_steps += 1;
                    moved = true;
                    accumulate.push((scan, left, left_steps));
                }

                if self.tiles[scan][right + 1] == '.' {
                    right += 1;
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
        ret
    }
    fn move_two(&self, row: usize, col: usize) -> Vec<(usize, usize, u64)> {
        let mut ret: Vec<(usize, usize, u64)> = Vec::new();

        if row != 1 {
            return ret;
        }

        let amphipod_type = self.tiles[row][col];

        let dest_col = destination_col(amphipod_type);

        let mut state = '.';

        let last_row = self.tiles.len() - 2;

        let mut lowest_dot_row = None;

        for scan_row in 2..=last_row{
            let tile = self.tiles[scan_row][dest_col];

            if state == '.' && tile == '.' {
                lowest_dot_row = Some(scan_row);
            } else if state == '.' && tile == amphipod_type {
                state = tile;
            } else if state == amphipod_type && tile == amphipod_type {
            } else {
                return ret;
            }
        }

        if let Some(row) = lowest_dot_row {
            assert!(dest_col != col);
            let go_left = dest_col < col;
            let mut scan = col;
            let mut steps = 0;

            loop {
                scan = if go_left { scan - 1 } else { scan + 1 };

                if self.tiles[1][scan] != '.' {
                    return ret;
                }

                steps += 1;

                if scan == dest_col {
                    break;
                }
            }
            let cost = (steps + row - 1).try_into().unwrap();
            ret.push((row, dest_col, cost));
        }
        ret
    }
    fn done(&self) -> bool {
        for col in [3, 5, 7, 9] {
            for row in 2..self.tiles.len() - 1 {
                let tile = self.tiles[row][col];

                if !home(col, tile) {
                    return false;
                }
            }
        }
        true
    }
}

fn recurse(board: &Board, best: &mut u64, curr: u64) {
    if board.done() {
        if curr < *best {
            // No move one and no move two, must be done already.
            *best = curr;
        }
        return;
    }

    /* Well this sucks. Whither clone, etc. */
    let mut state = Board { tiles: Vec::new() };
    for row in 0..board.tiles.len() {
        state.tiles.push(Vec::new());
        for col in 0..board.tiles[row].len() {
            state.tiles[row].push(board.tiles[row][col]);
        }
    }

    let amphipods = board.get_ordered_amphipods();

    for (row, col, amphipod_type) in amphipods {
        let mut try_move = board.move_one(row, col);

        if try_move.is_empty() {
            try_move = board.move_two(row, col);
        }

        if try_move.is_empty() {
        } else {
            for (dest_row, dest_col, steps) in try_move {
                let cost = curr + steps * move_cost(amphipod_type);

                if cost < *best {
                    state.tiles[row][col] = '.';
                    state.tiles[dest_row][dest_col] = amphipod_type;

                    recurse(&state, best, cost);

                    state.tiles[dest_row][dest_col] = '.';
                    state.tiles[row][col] = amphipod_type;
                }
            }
        }
    }
}

fn main() {
    let mut board = Board { tiles: Vec::new() };
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        board.tiles.push(line.chars().collect());
    }
    let mut best = u64::MAX;
    recurse(&board, &mut best, 0);
    println!("{best}");
}
