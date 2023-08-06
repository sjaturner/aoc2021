use std::io::{self, BufRead};

fn nibble(character: char) -> u32 {
    match character {
        'A'..='F' => character as u32 - 'A' as u32 + 0xa,
        'a'..='a' => character as u32 - 'a' as u32 + 0xa,
        '0'..='9' => character as u32 - '0' as u32 + 0x0,
        _ => todo!(),
    }
}

fn line_to_bitstring(line: &str) -> Vec<u32> {
    let mut ret = Vec::new();
    for character in line.chars() {
        let mut bit = 1u32 << 3;
        let nibble = nibble(character);

        while bit != 0 {
            ret.push(if bit & nibble != 0 { 1 } else { 0 });

            bit >>= 1;
        }
    }
    ret
}

struct Chunk<'a> {
    bits: &'a [u32],
    offs: usize,
    rems: i32,
}

fn tobin(chunk: &mut Chunk, len: i32) -> u32 {
    let mut ret: u32 = 0;
    let mut len = len;

    loop {
        ret |= chunk.bits[chunk.offs];
        chunk.offs += 1;
        chunk.rems -= 1;
        len -= 1;

        if len <= 0 {
            break;
        }
        ret <<= 1;
    }
    return ret;
}

fn literal(chunk: &mut Chunk) -> u32 {
    let mut ret = 0;
    loop {
        let cont = tobin(chunk, 1) == 1;
        ret |= tobin(chunk, 4);
        if !cont {
            break;
        }
        ret <<= 4;
    }
    ret
}

fn indent(depth: u32) {
    for _ in 0..depth {
        print!("    ");
    }
}

fn process(chunk: &mut Chunk, depth: u32, ver_sum: &mut u32) {
    let ver = tobin(chunk, 3);
    let typ = tobin(chunk, 3);
    indent(depth);
    println!("ver: {}", ver);
    *ver_sum += ver;
    indent(depth);
    println!("typ: {}", typ);
    match typ {
        4 => {
            // Literal
            indent(depth);
            println!("literal: {}", literal(chunk));
        }
        _ => {
            // Operator
            let ltid = tobin(chunk, 1);
            match ltid {
                0 => {
                    let offs = chunk.offs;
                    let total_length_of_subpackets = tobin(chunk, 15) as usize;
                    indent(depth);
                    println!("{}", total_length_of_subpackets);

                    while chunk.offs < offs + total_length_of_subpackets {
                        process(chunk, depth + 1, ver_sum);
                    }
                }
                1 => {
                    let total_number_of_subpackets = tobin(chunk, 11);
                    indent(depth);
                    println!("{}", total_number_of_subpackets);

                    for _ in 0..total_number_of_subpackets {
                        process(chunk, depth + 1, ver_sum);
                    }
                }
                _ => todo!(),
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let bitstring = line_to_bitstring(&line);
        println!("{line}");
        println!("{:?}", line_to_bitstring(&line));
        let mut ver_sum = 0;
        process(
            &mut Chunk {
                bits: &bitstring,
                offs: 0,
                rems: bitstring.len() as i32,
            },
            0,
            &mut ver_sum,
        );
        println!("ver_sum: {ver_sum}");
    }
}