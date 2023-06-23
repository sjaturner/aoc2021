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

    let args: Vec<String> = env::args().collect();
    println!("{}", args[1].parse::<u32>().unwrap());

    for day in 1..=args[1].parse::<u32>().unwrap() {
        let len = fish_state.len();
        for index in 0..len {
            if fish_state[index] == 0 {
                fish_state.push(8);
                fish_state[index] = 6;
            } else {
                fish_state[index] -= 1;
            }
        }
        if false {
            println!("{day} {:?}", fish_state);
        }
    }
    println!("{}", fish_state.len());
}
