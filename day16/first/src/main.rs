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

fn process(bits: &Vec<u32>, pos: usize, depth: u32) {
    let mut pos = pos;
    let ver = tobin(&bits[pos..pos + 3]);
    pos += 3;
    let typ = tobin(&bits[pos..pos + 3]);
    println!("{}", ver);
    println!("{}", typ);
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
