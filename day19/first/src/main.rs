use ndarray::prelude::*;
use ndarray::{concatenate, stack, Axis};
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut scanners: Vec<Array2<i32>> = Vec::new();
    let scanner_number: usize = 0;

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let line = line.trim();

        if line.len() == 0 {
        } else if line.len() >= 3 && line[0..3] == "---".to_string() {
            scanner_number = line.split(' ').collect::<Vec<&str>>()[2]
                .parse::<usize>()
                .unwrap();
            assert!(scanner_number == scanners.len());
            scanners.push(Array2::zeros((3, 0)))
        } else {
            let vals: Vec<i32> = line
                .split(',')
                .map(|chunk| chunk.parse::<i32>().unwrap())
                .collect();
            let nda = scanners[scanner_number];

//          nda.stack((vals[0], vals[1], vals[2]));
        }
    }
}
