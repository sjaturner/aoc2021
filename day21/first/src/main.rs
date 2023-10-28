use std::env;
fn main() {
    let mut args: Vec<_> = env::args().collect();
    args.remove(0);

    let mut pos: Vec<_> = args.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    let mut score: Vec<_> = args.iter().map(|_| 0).collect();
    let mut dice = 0;
    let mut player = 0;
    let mut rolls = 0;

    'outer: loop {
        for _ in 0..3 {
            let roll = dice + 1;

            pos[player] += roll % 10;
            pos[player] = if pos[player] > 10 {
                pos[player] - 10
            } else {
                pos[player]
            };

            dice += 1;
            dice %= 100;
            rolls += 1;
        }
        score[player] += pos[player];

        let done = score[player] >= 1000;
        player += 1;
        player %= pos.len();
        if done {
            break 'outer;
        }
    }
    println!("{}", rolls * score[player]);
}
