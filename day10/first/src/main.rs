use std::io::{self, BufRead};

fn closer(open: char) -> char {
    return match open {
        '[' => ']',
        '(' => ')',
        '{' => '}',
        '<' => '>',
        _ => panic!("cannot close {}", open),
    };
}

fn opener(open: char) -> char {
    return match open {
        ']' => '[',
        ')' => '(',
        '}' => '{',
        '>' => '<',
        _ => panic!("cannot close {}", open),
    };
}

fn illegal_score(open: char) -> u32 {
    return match open {
        ']' => 57,
        ')' => 3,
        '}' => 1197,
        '>' => 23517,
        _ => panic!("cannot close {}", open),
    };
}

fn balance(stack: &mut Vec<char>, close: char) {
    if let Some(character) = stack.pop() {
        if character != opener(close) {
            println!("expected {} saw {}", closer(character), close);
        }
    }
}
fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let mut stack = Vec::new();
        for bracket in line.chars() {
            match bracket {
                '[' | '(' | '{' | '<' => stack.push(bracket),
                ']' | ')' | '}' | '>' => balance(&mut stack, bracket),
                _ => panic!("did not expect {}", bracket),
            }
        }
    }
}
