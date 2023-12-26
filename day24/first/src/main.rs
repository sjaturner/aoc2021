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

fn f(state: i64, inp: i64, k0: i64, k1: i64, k2: i64) -> i64 {
    let w = inp;
    let mut z = state;
    let mut x = z;

    //  mod x 26
    assert!(x >= 0);
    x %= 26;

    //  div z k0
    assert!(k0 != 0);
    z /= k0;

    //  add x k1
    x += k1;

    //  eql x w
    x = if x == w { 1 } else { 0 };

    //  eql x 0
    x = if x == 0 { 1 } else { 0 };

    //  mul y 0
    //  add y 25
    let mut y = 25;

    //  mul y x
    y *= x;

    //  add y 1
    y += 1;

    //  mul z y
    z *= y;

    //  mul y 0
    //  add y w
    y = w;

    //  add y k2
    y += k2;

    //  mul y x
    y *= x;

    //  add z y
    z += y;

    z
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

    let verbose = false;
    let mut ok = true;
    for instruction in &instructions {
        if verbose {
            println!("{:?}", instruction);
        }
        if !step(&mut state, &mut input, *instruction) {
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

    if ok {
        println!("   {:?}", state.regs[3]);
    }

    if false {
        println!("{:?}", instructions);
    }

    let mut z = 0;
    let mut input = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 9, 9, 9, 9, 9];

    z = f(z, input[0], 1, 13, 14);
    z = f(z, input[1], 1, 12, 8);
    z = f(z, input[2], 1, 11, 5);
    z = f(z, input[3], 26, 0, 4);
    z = f(z, input[4], 1, 15, 10);
    z = f(z, input[5], 26, -13, 13);
    z = f(z, input[6], 1, 10, 16);
    z = f(z, input[7], 26, -9, 5);
    z = f(z, input[9], 1, 11, 6);
    z = f(z, input[9], 1, 13, 13);
    z = f(z, input[10], 26, -14, 6);
    z = f(z, input[11], 26, -3, 7);
    z = f(z, input[12], 26, -2, 13);
    z = f(z, input[13], 26, -14, 3);

    println!("   {:?}", z);
}
