use std::io::{self, BufRead};

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
    crc.card * ELEMS_PER_CARD + crc.row * ELEMS_PER_ROW + crc.col
}

fn row_complete(marked: &Vec<bool>, index: usize) -> bool {
    let crc = index_to_crc(index);

    for col in 0..ELEMS_PER_ROW {
        let index = crc_to_index(CardRowCol {
            card: crc.card,
            row: crc.row,
            col,
        });

        if !marked[index] {
            return false;
        }
    }

    true
}

fn col_complete(marked: Vec<bool>, index: usize) -> bool {
    false
}

fn main() {
    let stdin = io::stdin();
    let mut caller: Vec<u32> = Vec::new();
    let mut entries: Vec<u32> = Vec::new();
    let mut marked: Vec<bool> = Vec::new();

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
                    entries.push(item.parse().unwrap());
                    marked.push(false);
                }
            }
        }
    }

    let cards = entries.len() / ELEMS_PER_CARD;
    assert!(entries.len() == cards * ELEMS_PER_CARD);

    let mut bingo_index: Option<usize> = None;
    'bingo: for number in caller {
        for index in 0..entries.len() {
            if number == entries[index] {
                println!("{number} {:?}", index_to_crc(index));
                marked[index] = true;
                if row_complete(&marked, index) && row_complete(&marked, index) {
                    bingo_index = Some(index);
                    break 'bingo;
                }
            }
        }
    }

    if let Some(bingo_index) = bingo_index {
        let card = index_to_crc(bingo_index).card;
        let sum = marked
            .into_iter()
            .enumerate()
            .filter(|&(index, _)| index_to_crc(index).card == card)
            .filter(|&(_, marked)| !marked)
            .map(|(index, _)| entries[index])
            .sum::<u32>();

        println!();
        println!(
            "{sum} {} {}",
            entries[bingo_index],
            sum * entries[bingo_index]
        );
    };
}
