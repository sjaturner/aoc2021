use std::env;

#[derive(Clone, Copy, Debug)]
struct Player {
    pos: u64,
    rem: u64,
    state: u64,
    acc: u64,
}

fn recurse(current: usize, players: [Player; 2], player_1_wins: &mut u64, player_2_wins: &mut u64) {
    if true {
        println!(
            "{current} {:?} {} {}",
            players, 0, 0, // *player_1_wins, *player_2_wins
        );
    }
    match players[current].state {
        0..=2 => {
            let mut copy_players = players;
            copy_players[current].state += 1;
            for _roll in 1..=3 {
                copy_players[current].acc += 1;
                recurse(current, copy_players, player_1_wins, player_2_wins);
            }
        }
        3 => {
            let next = (((players[current].pos + players[current].acc) - 1) % 10) + 1;
            if next >= players[current].rem {
                if current == 0 {
                    *player_1_wins += 1
                } else {
                    *player_2_wins += 1
                };
            } else {
                let mut copy_players = players;
                copy_players[current].pos = next;
                copy_players[current].rem -= next;
                copy_players[current].state = 0;
                copy_players[current].acc = 0;

                recurse(
                    if current == 0 { 1 } else { 0 },
                    copy_players,
                    player_1_wins,
                    player_2_wins,
                );
            }
        }
        _ => assert!(false),
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let total = args[1].parse::<u64>().unwrap();
    let player_1 = args[2].parse::<u64>().unwrap();
    let player_2 = args[3].parse::<u64>().unwrap();
    let players = [
        Player {
            pos: player_1,
            rem: total,
            state: 0,
            acc: 0,
        },
        Player {
            pos: player_2,
            rem: total,
            state: 0,
            acc: 0,
        },
    ];
    let mut player_1_wins = 0;
    let mut player_2_wins = 0;

    recurse(0, players, &mut player_1_wins, &mut player_2_wins);

    println!("{} {}", player_1_wins, player_2_wins);
}
