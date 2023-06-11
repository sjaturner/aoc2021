use std::io::{self, BufRead};

struct Entry {
    value: u32,
    marked: bool,
}

fn main() {
    let stdin = io::stdin();
    let mut caller: Vec<u32> = Vec::new();
    let mut cards: Vec<Vec<Vec<Entry>>> = Vec::new();
    let mut this_in_card = false;
    let mut last_in_card = false;

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        //      println!("{}", line);

        if caller.len() == 0 {
            caller = line.trim().split(',').map(|s| s.parse().unwrap()).collect();
            println!("{:?}", caller);
        } else {
            let row: Vec<_> = line.trim().split_whitespace().collect(); // .map(|s| s.parse().unwrap()).collect();

            if row.len() == 0 {
                this_in_card = false;
            } else {
                this_in_card = true;

                if !last_in_card && this_in_card {
                    cards.push(Vec::new())
                }

                for item in &row {
                    let entry = Entry {
                        value: item.parse().unwrap(),
                        marked: false,
                    };
                    cards[&cards.len() - 1][&cards[0].len() - 1].push(entry);
                }
            }

            last_in_card = this_in_card;
            println!("{} {:?}", row.len(), row)
        }
    }
}
