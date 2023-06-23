use std::env;
use std::io;

fn main() {
    let reader = io::stdin();
    let mut buffer: String = String::new();

    reader
        .read_line(&mut buffer)
        .ok()
        .expect("error")
        .to_string();

    let mut fish_state = buffer
        .trim()
        .split(',')
        .map(|val| val.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    println!("{:?}", fish_state);
}
