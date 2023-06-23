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

    let fish_state = buffer
        .trim()
        .split(',')
        .map(|val| val.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    if false {
        println!("{:?}", fish_state);
    }

    let args: Vec<String> = env::args().collect();

    if false {
        println!("{}", args[1].parse::<u32>().unwrap());
    }

    let mut fish_buckets: [u64; 9] = [0; 9];

    for state in &fish_state {
        fish_buckets[*state as usize] += 1;
    }

    for _ in 1..=args[1].parse::<u32>().unwrap() {
        let mut next_buckets: [u64; 9] = [0; 9];

        if false {
            println!("{:?}", fish_buckets);
        }

        let store = fish_buckets[0];
        for bucket in 0..(fish_buckets.len() - 1) {
            next_buckets[bucket] = fish_buckets[bucket + 1];
        }
        next_buckets[6] += store;
        next_buckets[8] = store;
        fish_buckets.copy_from_slice(&next_buckets)
    }

    println!("{}", fish_buckets.into_iter().sum::<u64>());
}
