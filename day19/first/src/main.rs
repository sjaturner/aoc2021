use ndarray::prelude::*;
use ndarray::{concatenate, Axis};
use std::io::{self, BufRead};

fn make_transforms() -> Vec<Array<i32, Dim<[usize; 2]>>> {
    let mut ret = Vec::new();

    for s2 in [-1, 1] {
        for s1 in [-1, 1] {
            for s0 in [-1, 1] {
                let a = Array2::from_diag(&arr1(&[s2, s1, s0]));
                for r0 in [0, 1, 2] {
                    for r1 in [0, 1, 2] {
                        if r1 == r0 {
                            continue;
                        }
                        for r2 in [0, 1, 2] {
                            if r2 == r0 {
                                continue;
                            }
                            if r2 == r1 {
                                continue;
                            }
                            let mut b = Array::<i32, _>::zeros((3, 3));
                            b[[0, r0]] = 1;
                            b[[1, r1]] = 1;
                            b[[2, r2]] = 1;

                            ret.push(a.dot(&b))
                        }
                    }
                }
            }
        }
    }
    ret
}

fn append_xyzs_to_cloud(cloud: &mut Array<i32, Dim<[usize; 2]>>, xyzs: Vec<(i32, i32, i32)>) {
    for tuple in xyzs {
        *cloud = concatenate![Axis(0), *cloud, array![[tuple.0, tuple.1, tuple.2],]];
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanners: Vec<Vec<(i32, i32, i32)>> = Vec::new();
    let mut build = Vec::new();
    let transforms = make_transforms();

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let line = line.trim();

        if line.len() == 0 {
        } else if line.len() >= 3 && line[0..3] == "---".to_string() {
            if build.len() != 0 {
                scanners.push(build.to_vec());
                build.clear();
            }
            // Gets the scanner number, not immediately useful.
            let _ = line.split(' ').collect::<Vec<&str>>()[2]
                .parse::<usize>()
                .unwrap();
        } else {
            let vals: Vec<i32> = line
                .split(',')
                .map(|chunk| chunk.parse::<i32>().unwrap())
                .collect();
            build.push((vals[0], vals[1], vals[2]));
        }
    }
    scanners.push(build.to_vec());
    build.clear();

    println!("{:?}", transforms);

    let mut cloud = Array::zeros((0, 3));
    println!("{:?}", cloud);
    append_xyzs_to_cloud(&mut cloud, scanners.remove(0));
    println!("{:?}", cloud);
}
