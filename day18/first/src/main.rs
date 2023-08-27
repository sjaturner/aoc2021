use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Open,
    Close,
    Val(i32),
}

fn explode(ip: &Vec<Token>) -> Option<Vec<Token>> {
    let mut result = Vec::new();
    let mut exploded = false;
    for index in 0..ip.len() {
        match ip[index] {
            Token::Open | Token::Close => {
                result.push(ip[index]);
            }
            Token::Val(val) => {
                if val <= 9 || exploded {
                    result.push(ip[index]);
                } else {
                    let a = val / 2 - 1;
                    let b = val / 2 + 1;
                    result.push(Token::Open);
                    result.push(Token::Val(a));
                    result.push(Token::Val(b));
                    result.push(Token::Close);
                    exploded = true;
                }
            }
        }
    }

    if exploded {
        Some(result)
    } else {
        None
    }
}

fn reduce(ip: &Vec<Token>) -> Option<Vec<Token>> {
    let mut nesting = 0;
    let mut num_first: Option<usize> = None;
    let mut triggered: Option<usize> = None;
    let mut num_last: Option<usize> = None;

    for index in 0..ip.len() {
        if triggered == None {
            nesting += if ip[index] == Token::Open {
                1
            } else if ip[index] == Token::Close {
                -1
            } else {
                0
            };

            if let Token::Val(_) = ip[index] {
                num_first = Some(index);
            }

            if nesting >= 5 {
                if index < ip.len() - 4 {
                    match ip[index..index + 4] {
                        [Token::Open, Token::Val(_), Token::Val(_), Token::Close] => {
                            triggered = Some(index);
                        }
                        _ => {}
                    }
                }
            }
        } else if num_last == None {
            let trigger_index = triggered.unwrap();

            if index >= trigger_index + 4 {
                if let Token::Val(_) = ip[index] {
                    num_last = Some(index);
                }
            }
        }
    }

    if triggered == None {
        None
    } else {
        let mut result = Vec::new();
        let trigger_index = triggered.unwrap();

        for index in 0..ip.len() {
            if index == trigger_index {
                result.push(Token::Val(0));
            } else if index > trigger_index && index < trigger_index + 4 {
            } else if Some(index) == num_first {
                if let Token::Val(a) = ip[trigger_index + 1] {
                    if let Token::Val(b) = ip[index] {
                        result.push(Token::Val(a + b));
                    }
                }
            } else if Some(index) == num_last {
                if let Token::Val(a) = ip[trigger_index + 2] {
                    if let Token::Val(b) = ip[index] {
                        result.push(Token::Val(a + b));
                    }
                }
            } else {
                result.push(ip[index]);
            }
        }

        Some(result)
    }
}

fn render(ip: &Vec<Token>) {
    for token in ip {
        match token {
            Token::Open => {
                print!("[");
            }
            Token::Close => {
                print!("]");
            }
            Token::Val(number) => {
                print!("{number},");
            }
        }
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut tokens = Vec::new();
        let line = line.expect("Could not read line from standard in");

        for c in line.chars() {
            match c {
                '[' => {
                    tokens.push(Token::Open);
                }
                ']' => {
                    tokens.push(Token::Close);
                }
                '0'..='9' => tokens.push(Token::Val(c as i32 - '0' as i32)),
                ',' => {}
                _ => {
                    todo!();
                }
            }
        }

        loop {
            if let Some(reduced) = reduce(&tokens) {
                tokens = reduced;
            } else if let Some(exploded) = explode(&tokens) {
                tokens = exploded;
            } else {
                break;
            }
        }
        render(&tokens);
    }
}
