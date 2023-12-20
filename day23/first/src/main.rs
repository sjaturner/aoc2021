use std::io::{self, BufRead};

struct Board {
    tile: Vec<Vec<char>>,
}

impl Board {
    fn show(self) {
        for row in self.tile {
            for elem in row {
                print!("{elem}");
            }
            println!();
        }
    }
}

fn main() {
    let mut board = Board {
        tile: Vec::new()
    };
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        println!("{}", line);
        board.tile.push(line.chars().collect());
    }
    board.show();
}
