use std::io::{self, BufRead};
use rand::Rng;

#[derive(Debug, Copy, Clone)]
enum Register {
    W,
    X,
    Y,
    Z,
}

fn register_to_index(reg: Register) -> usize {
    match reg {
        Register::W => 0,
        Register::X => 1,
        Register::Y => 2,
        Register::Z => 3,
    }
}

fn str_to_register(s: &str) -> Register {
    match s {
        "w" => Register::W,
        "x" => Register::X,
        "y" => Register::Y,
        "z" => Register::Z,

        _ => panic!(),
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum OpCode {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

fn str_to_opcode(s: &str) -> OpCode {
    match s {
        "inp" => OpCode::Inp,
        "add" => OpCode::Add,
        "mul" => OpCode::Mul,
        "div" => OpCode::Div,
        "mod" => OpCode::Mod,
        "eql" => OpCode::Eql,

        _ => panic!(),
    }
}

#[derive(Debug, Copy, Clone)]
enum Src {
    Immediate(i64),
    Register(Register),
    Input,
}

fn str_to_src(s: &str) -> Src {
    match s.parse::<i64>() {
        Ok(val) => Src::Immediate(val),
        Err(_) => Src::Register(str_to_register(s)),
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    opcode: OpCode,
    dst: Register,
    src: Src,
}

#[derive(Debug)]
struct State {
    regs: [i64; 4],
}

fn step(state: &mut State, input: &mut Vec<i64>, instruction: Instruction) -> bool {
    let src_val = match instruction.src {
        Src::Input => input.remove(0),
        Src::Immediate(val) => val,
        Src::Register(reg) => state.regs[register_to_index(reg)],
    };
    let dst: &mut i64 = &mut state.regs[register_to_index(instruction.dst)];
    if ((instruction.opcode == OpCode::Div) && src_val == 0)
        || (instruction.opcode == OpCode::Mod && (*dst < 0 || src_val <= 0))
    {
        false
    } else {
        *dst = match instruction.opcode {
            OpCode::Inp => src_val,
            OpCode::Add => *dst + src_val,
            OpCode::Mul => *dst * src_val,
            OpCode::Div => *dst / src_val,
            OpCode::Mod => *dst % src_val,
            OpCode::Eql => {
                if *dst == src_val {
                    1
                } else {
                    0
                }
            }
        };
        true
    }
}

fn g(z: i64, inp: i64, k0: i64, k1: i64, k2: i64) -> i64 {
    let mut x = z % 26 + k1; // Sometimes the choice of k1 will mean that the x comparison below must fail as inp is 1..9

    let mut z = z;

    if x == inp {
        z /= k0; // And k0 is either 1 or 26
    } else {
        // The modulo 26 part will be lost with these statements here as k0 is 1 or 26
        z /= k0; // And k0 is either 1 or 26
        z *= 26;

        z += inp + k2;
    }

    z
}

fn bcd_array(val: i64) -> Vec<i64> {
    let mut ret = Vec::new();
    let mut val = val;

    for _ in 1..=14 {
        ret.push(val % 10);
        val /= 10;
    }
    ret.reverse();
    ret
}

fn main() {
    let stdin = io::stdin();
    let mut instructions = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let line: Vec<&str> = line.split_whitespace().collect();
        let instruction = Instruction {
            opcode: str_to_opcode(line[0]),
            dst: str_to_register(line[1]),
            src: if line.len() == 2 {
                Src::Input
            } else {
                str_to_src(line[2])
            },
        };
        instructions.push(instruction);
    }
    let mut state = State { regs: [0; 4] };
    let mut input = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 9, 9, 9, 9, 9];

    if false {
        println!("{:?}", instructions);
    }

    let mut rng = rand::thread_rng();

    for val in 0..99999999999999i64 {
        let val = rng.gen_range(0..99999999999999i64);
        let input = bcd_array(val);

        let mut state = State { regs: [0; 4] };
        let mut mut_input = input.clone();
        let verbose = false;
        let mut ok = true;
        for instruction in &instructions {
            if verbose {
                println!("{:?}", instruction);
            }
            if !step(&mut state, &mut mut_input, *instruction) {
                if verbose {
                    println!("fault");
                }
                ok = false;
                break;
            }
            if verbose {
                println!("   {:?}", state);
            }
        }

        if ok && verbose{
            println!("a  {:?}", state.regs[3]);
        }

        let ks: [[i64; 3]; 14] = [
            [1, 13, 14],
            [1, 12, 8],
            [1, 11, 5],
            [26, 0, 4],
            [1, 15, 10],
            [26, -13, 13],
            [1, 10, 16],
            [26, -9, 5],
            [1, 11, 6],
            [1, 13, 13],
            [26, -14, 6],
            [26, -3, 7],
            [26, -2, 13],
            [26, -14, 3],
        ];

        let mut z = 0;
        for i in 0..14 {
            z = g(z, input[i], ks[i][0], ks[i][1], ks[i][2]);
        }

        if z != state.regs[3] {
            panic!();
        }

        if verbose {
            println!("c  {:?}", z);
        }

        if false && z == 0 {
            println!("woot");
            panic!();
        }

        if val % 1000 == 0 {
            print!(".");
        }
    }
}
