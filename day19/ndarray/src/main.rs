use ndarray::prelude::*;

fn main() {
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

                            println!("{:?}", a.dot(&b));
                            println!();
                        }
                    }
                }
            }
        }
    }
}
