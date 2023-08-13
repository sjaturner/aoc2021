struct Range {
    lo: i32,
    hi: i32,
}
struct Target {
    x: Range,
    y: Range,
}
#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}
fn step_x(vel: &mut i32, pos: &mut i32) {
    *pos += *vel;

    *vel = if *vel > 0 {
        *vel - 1
    } else if *vel < 0 {
        *vel + 1
    } else {
        *vel
    };
}
fn step_y(vel: &mut i32, pos: &mut i32) {
    *pos += *vel;

    *vel -= 1;
}
fn step(vel: &mut Velocity, pos: &mut Position) {
    step_x(&mut vel.x, &mut pos.x);
    step_y(&mut vel.y, &mut pos.y);
}

fn main() {
    let target = Target {
        x: Range { lo: 20, hi: 30 },
        y: Range { lo: -10, hi: -5 },
    };
    let mut candidates_x = Vec::new();

    for velx in 1..=target.x.hi + 1 {
        let mut vel = Velocity { x: velx, y: 0 };
        let mut pos = Position { x: 0, y: 0 };
        let mut last_x = 0;

        while pos.x <= target.x.hi + 1 {
            if pos.x >= target.x.lo && pos.x <= target.x.hi {
                candidates_x.push(velx);
                break;
            }
            step_x(&mut vel.x, &mut pos.x);
            if last_x == pos.x {
                break;
            }
            last_x = pos.x;
        }
    }
    println!("{:?}", candidates_x);
}
