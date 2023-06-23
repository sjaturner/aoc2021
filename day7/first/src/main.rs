use std::io;

fn main() {
    let reader = io::stdin();
    let mut buffer: String = String::new();

    reader
        .read_line(&mut buffer)
        .ok()
        .expect("error")
        .to_string();

    let crab_location = buffer
        .trim()
        .split(',')
        .map(|val| val.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    if false {
        println!("{:?}", crab_location);
    }

    let min = crab_location.iter().min().unwrap();
    let max = crab_location.iter().max().unwrap();

    let mut best_location: Option<i32> = None;
    let mut lowest_score: Option<i32> = None;
    for location in *min..=*max {
        let score = crab_location
            .iter()
            .map(|x| i32::abs(x - location))
            .sum::<i32>();

        if let Some(current) = lowest_score {
            if score == current {
                assert!(false);
            } else if score < current {
                lowest_score = Some(score);
                best_location = Some(location);
            }
        } else {
            lowest_score = Some(score);
        }
    }

    if let Some(location) = best_location {
        if let Some(score) = lowest_score {
            println!("{location} {score}");
        }
    }
}
