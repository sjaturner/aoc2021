use std::io::{self, BufRead};
use std::{cell::RefCell, rc::Rc};

type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug, Clone)]
pub enum Elem<'a> {
    Val(u32),
    Pair(&'a TreeNode),
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
fn build_elem_val(input: &[u8], pos: &mut usize) -> Elem {
    Elem::Val(0)
}

fn build_elem_pair(input: &[u8], pos: &mut usize) -> Elem {
    Elem::Val(0)
}

fn build_tree_node(input: &[u8], pos: &mut usize) -> TreeNode {
    eat_space(input, pos);

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
