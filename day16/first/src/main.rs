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

fn tobin(bits: &[u32]) -> u32 {
    let mut ret: u32 = 0;
    let mut count = bits.len();
    let mut index = 0;

    loop {
        ret |= bits[index];
        count -= 1;
        if count <= 0 {
            break;
        }
        ret <<= 1;
        index += 1;
    }
    return ret;
}

fn literal(bits: &[u32]) -> u32 {
    let mut offs = 0;
    let mut ret = 0;
    loop {
        let cont = bits[offs] == 1;
        offs += 1;
        ret |= tobin(&bits[offs..offs + 4]);

        if !cont {
            break;
        }

        ret <<= 4;
        offs += 4;
    }
    ret
}

fn process(bits: &Vec<u32>, pos: usize, depth: u32) {
    let mut pos = pos;
    let ver = tobin(&bits[pos..pos + 3]);
    pos += 3;
    let typ = tobin(&bits[pos..pos + 3]);
    pos += 3;
    println!("{}", ver);
    println!("{}", typ);
    match typ {
        4 => {
            // Literal
            println!("literal: {}", literal(&bits[pos..]));
        }
        _ => {
            // Operator
            let ltid = bits[pos];
            pos += 1;

            match ltid {
                0 => {
                    let total_length_of_subpackets = tobin(&bits[pos..pos + 15]);
                    pos += 15;
                    println!("{}", total_length_of_subpackets);
                }
                1 => {
                    let total_number_of_subpackets = tobin(&bits[pos..pos + 11]);
                    pos += 11;
                    println!("{}", total_number_of_subpackets);
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
        process(&bitstring, 0, 0);
    }
}
