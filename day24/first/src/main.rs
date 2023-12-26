use std::io::{self, BufRead};

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

    for instruction in &instructions {
        println!("{:?}", instruction);
        if !step(&mut state, &mut input, *instruction) {
            println!("fault");
            break;
        }
        println!("   {:?}", state);
    }

    println!("{:?}", instructions);
}
