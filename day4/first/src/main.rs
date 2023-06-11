use std::io::{self, BufRead};

struct Entry {
    value: u32,
    marked: bool,
}

const ELEMS_PER_ROW: usize = 5;
const ROWS_PER_CARD: usize = 5;
const ELEMS_PER_CARD: usize = ELEMS_PER_ROW * ROWS_PER_CARD;

fn index_to_crc(index: usize, card: &mut usize, row: &mut usize, col: &mut usize) {
    let mut pos = index;
    *card = index / ELEMS_PER_CARD;
    pos -= *card * ELEMS_PER_CARD;
    *row = pos / ELEMS_PER_ROW;
    pos -= *row * ELEMS_PER_ROW;
    *col = pos;
}

fn row_complete(items: Vec<Entry>, index: usize) -> bool {
    false
}

fn col_complete(items: Vec<Entry>, index: usize) -> bool {
    false
}

fn main() {
    let stdin = io::stdin();
    let mut caller: Vec<u32> = Vec::new();
    let mut items: Vec<Entry> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        //      println!("{}", line);

        if caller.len() == 0 {
            caller = line.trim().split(',').map(|s| s.parse().unwrap()).collect();
            println!("{:?}", caller);
        } else {
            let row: Vec<_> = line.trim().split_whitespace().collect();

            if row.len() == 0 {
            } else {
                for item in &row {
                    items.push(Entry {
                        value: item.parse().unwrap(),
                        marked: false,
                    });
                }
            }
        }
    }
    let cards = items.len() / ELEMS_PER_CARD;

    println!("{cards}");

    assert!(items.len() == cards * ELEMS_PER_CARD);

    for number in caller {
        for index in items.iter().position(|item| item.value == number) {
            println!("{number} {}", index);
        }
        println!();
    }
}
