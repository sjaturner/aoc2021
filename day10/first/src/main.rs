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
        '>' => 25137,
        _ => panic!("cannot close {}", open),
    };
}

fn balance(stack: &mut Vec<char>, close: char) -> u32 {
    if let Some(character) = stack.pop() {
        if character != opener(close) {
            if false {
                println!("expected {} saw {}", closer(character), close);
            }
            return illegal_score(close);
        }
    }
    0
}
fn main() {
    let stdin = io::stdin();
    let mut sum = 0;

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let mut stack = Vec::new();
        'line: for bracket in line.chars() {
            match bracket {
                '[' | '(' | '{' | '<' => stack.push(bracket),
                ']' | ')' | '}' | '>' => {
                    let result = balance(&mut stack, bracket);
                    if result != 0 {
                        sum += result;
                        break 'line;
                    }
                }
                _ => panic!("did not expect {}", bracket),
            }
        }
    }
    println!("{sum}");
}
