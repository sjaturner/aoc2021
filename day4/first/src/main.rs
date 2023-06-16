use std::io::{self, BufRead};

struct Entry {
    value: u32,
    marked: bool,
}

const ELEMS_PER_ROW: usize = 5;
const ROWS_PER_CARD: usize = 5;
const ELEMS_PER_CARD: usize = ELEMS_PER_ROW * ROWS_PER_CARD;

#[derive(Debug)]
struct CardRowCol {
    card: usize,
    row: usize,
    col: usize,
}

fn index_to_crc(index: usize) -> CardRowCol {
    let mut pos = index;
    let card = index / ELEMS_PER_CARD;
    pos -= card * ELEMS_PER_CARD;
    let row = pos / ELEMS_PER_ROW;
    pos -= row * ELEMS_PER_ROW;
    let col = pos;

    CardRowCol { card, row, col }
}

fn crc_to_index(crc: CardRowCol) -> usize {
    0
}

fn row_complete(items: Vec<Entry>, index: usize) -> bool {
    let crc = index_to_crc(index);

    for col in 0..ELEMS_PER_ROW {
        let index = crc_to_index(CardRowCol {
            card: crc.card,
            row: crc.row,
            col,
        });

        let entry = &items[index];
    }

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
        for (index, value) in items
            .iter()
            .enumerate()
            .filter(|(_, item)| item.value == number)
        {
            println!("{number} {:?}", index_to_crc(index));
        }
    }
}
