use std::io::{self, BufRead};

#[derive(Debug)]
enum Register {
    W,
    X,
    Y,
    Z,
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

#[derive(Debug)]
enum OpCode {
    INP,
    ADD,
    MUL,
    DIV,
    MOD,
    EQL,
}

fn str_to_opcode(s: &str) -> OpCode {
    match s {
        "inp" => OpCode::INP,
        "add" => OpCode::ADD,
        "mul" => OpCode::MUL,
        "div" => OpCode::DIV,
        "mod" => OpCode::MOD,
        "eql" => OpCode::EQL,

        _ => panic!(),
    }
}

#[derive(Debug)]
enum Src {
    Immediate(i32),
    Register(Register),
    Input,
}

fn str_to_src(s: &str) -> Src {
    match s.parse::<i32>()
    {
        Ok(val) => Src::Immediate(val),
        Err(_) => {
            Src::Register(str_to_register(s))
        },
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: OpCode,
    dst: Register,
    src: Src,
}

fn main() {
    let stdin = io::stdin();
    let mut instructions = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let line: Vec<&str> = line.split_whitespace().collect();
        let mut instruction = Instruction {
            opcode: str_to_opcode(line[0]),
            dst: str_to_register(line[1]),
            src: if line.len() == 2 { Src::Input } else { str_to_src(line[2])},
        };
        instructions.push(instruction);
    }
    println!("{:?}", instructions);
}
