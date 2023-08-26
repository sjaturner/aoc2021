use std::io::{self, BufRead};
use std::{cell::RefCell, rc::Rc};

type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug, Clone)]
pub enum Elem {
    Val(u32),
    Pair(Rc<RefCell<TreeNode>>),
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    l: Elem,
    r: Elem,
}

fn char_at(input: &[u8], pos: &mut usize) -> char {
    input[*pos] as char
}

fn eat_space(input: &[u8], pos: &mut usize) {
    while char_at(input, pos).is_whitespace() {
        *pos += 1;
    }
}
fn build_elem(input: &[u8], pos: &mut usize) -> Elem {
    eat_space(input, pos);
    match char_at(input, pos) {
        '0'..='9' => Elem::Val(0),
        '[' => Elem::Val(0),
        _ => {
            todo!();
        }
    }
}

fn build_tree_node(input: &[u8], pos: &mut usize) -> TreeNode {
    eat_space(input, pos);
    match char_at(input, pos) {
        '[' => {
            *pos += 1;
            build_elem(input, pos);
            eat_space(input, pos);
            assert!(char_at(input, pos) == ',');
            build_elem(input, pos);
            eat_space(input, pos);
            assert!(char_at(input, pos) == ']');
        }
        _ => {}
    }

    return TreeNode {
        l: Elem::Val(0),
        r: Elem::Val(0),
    };
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");

        println!("{line}");

        let tree = build_tree_node(line.as_bytes(), &mut 0);
    }
}
