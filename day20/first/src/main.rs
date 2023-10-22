use std::env;
use std::io::{self, BufRead};

fn border(block: Vec<Vec<u32>>, pad: usize) -> Vec<Vec<u32>> {
    let width = block[0].len();
    let empty_line = vec![0; 2 * pad + width];
    let start_end_padding = vec![0; pad];
    let mut ret = Vec::new();

    for _ in 0..pad {
        ret.push(empty_line.clone());
    }

    for line in block.clone() {
        let mut padded = Vec::new();
        padded.extend(start_end_padding.clone());
        padded.extend(line.clone());
        padded.extend(start_end_padding.clone());
        ret.push(padded);
    }

    for _ in 0..pad {
        ret.push(empty_line.clone());
    }

    ret
}

fn edging(block: &Vec<Vec<u32>>, trim: usize) -> Vec<Vec<u32>> {
    let height = block.len();
    let width = block[0].len();
    let mut ret = Vec::new();

    for y in 0..height {
        if y >= trim && y + trim < height {
            ret.push(block[y][trim..width - 1].to_vec());
        }
    }

    ret
}
fn get_pixel(block: &Vec<Vec<u32>>, coord: (i32, i32)) -> u32 {
    let width = block[0].len() as i32;
    let height = block.len() as i32;
    if coord.0 < 0 || coord.0 >= width {
        0
    } else if coord.1 < 0 || coord.1 >= height {
        0
    } else {
        block[coord.1 as usize][coord.0 as usize]
    }
}

fn get_offset(block: &Vec<Vec<u32>>, coord: (i32, i32)) -> usize {
    let mut accumulator = 0;
    for y in -1..=1 {
        for x in -1..=1 {
            let bit = get_pixel(block, (coord.0 + x, coord.1 + y));
            accumulator <<= 1;
            accumulator |= bit;
        }
    }
    accumulator as usize
}

fn transform(block: &Vec<Vec<u32>>, lut: &Vec<u32>) -> Vec<Vec<u32>> {
    let width = block[0].len();
    let height = block.len();
    let line: Vec<u32> = vec![0; width];
    let mut ret = Vec::new();

    for _ in 0..height {
        ret.push(line.clone());
    }

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            ret[y as usize][x as usize] = lut[get_offset(block, (x, y))];
        }
    }
    ret
}

fn render(block: &Vec<Vec<u32>>) {
    let width = block[0].len();
    let height = block.len();
    for y in 0..height {
        for x in 0..width {
            print!("{}", if block[y][x] == 1 { '#' } else { '.' });
        }
        println!();
    }
}

fn weigh(block: &Vec<Vec<u32>>) -> u32 {
    let width = block[0].len();
    let height = block.len();
    let mut ret = 0;
    for y in 0..height {
        for x in 0..width {
            ret += block[y][x];
        }
    }
    ret
}

fn main() {
    let stdin = io::stdin();
    let mut lut = Vec::new();
    let mut block = Vec::new();

    let args: Vec<_> = env::args().collect();

    let mut goes = 2;

    if args.len() > 1 {
        goes = args[1].parse::<u32>().unwrap();
        println!("{}", goes);
    }

    for (index, line) in stdin.lock().lines().enumerate() {
        let line = line.expect("Could not read line from standard in");
        let elems: Vec<u32> = line
            .chars()
            .map(|item| if item == '.' { 0 } else { 1 })
            .collect();

        if index == 0 {
            lut = elems;
        } else if elems.len() != 0 {
            block.push(elems)
        }
    }
    let edge = 4 * goes;
    block = border(block, edge as usize);
    render(&block);

    assert!(goes % 2 == 0);

    for _ in 0..goes / 2 {
        block = border(block, 2);
        block = transform(&block, &lut);
        block = transform(&block, &lut);
        block = edging(&block, 1);
        render(&block);
    }

    println!("{}", weigh(&block));
}
