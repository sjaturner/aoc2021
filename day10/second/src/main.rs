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

fn compete_score(bracket: char) -> u64 {
    return match bracket {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("cannot close {}", bracket),
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
    let mut results = Vec::new();

    for line in stdin.lock().lines() {
        let mut corrupted = false;
        let line = line.expect("Could not read line from standard in");
        let mut stack = Vec::new();
        'line: for bracket in line.chars() {
            match bracket {
                '[' | '(' | '{' | '<' => stack.push(bracket),
                ']' | ')' | '}' | '>' => {
                    let result = balance(&mut stack, bracket);
                    if result != 0 {
                        corrupted = true;
                        break 'line;
                    }
                }
                _ => panic!("did not expect {}", bracket),
            }
        }
        if !corrupted && stack.len() != 0 {
            let atoms = stack
                .iter()
                .rev()
                .map(|bracket| closer(*bracket))
                .map(|bracket| compete_score(bracket))
                .collect::<Vec<u64>>();
            let mut accumulator = 0u64;
            for atom in atoms {
                accumulator *= 5;
                accumulator += atom;
            }
            results.push(accumulator);
        }
    }
    results.sort();
    assert!(results.len() % 2 == 1);
    let midpoint = (results.len()) / 2;
    if false {
        println!("{:?}", results);
    }
    println!("   {:?}", results[midpoint]);
}
