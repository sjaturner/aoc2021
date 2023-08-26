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

fn build(input: &str, pos: &mut usize) -> TreeNode {
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
    }
}
