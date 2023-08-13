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
    let _target = Target {
        x: Range { lo: 20, hi: 30 },
        y: Range { lo: -10, hi: -5 },
    };
    let target = Target {
        x: Range { lo: 227, hi: 318 },
        y: Range { lo: -92, hi: -53 },
    };

    let mut candidates_velx = Vec::new();
    for velx in 1..=target.x.hi + 1 {
        let mut vel = Velocity { x: velx, y: 0 };
        let mut pos = Position { x: 0, y: 0 };
        let mut last_x = 0;

        while pos.x <= target.x.hi + 1 {
            if pos.x >= target.x.lo && pos.x <= target.x.hi {
                candidates_velx.push(velx);
                break;
            }
            step_x(&mut vel.x, &mut pos.x);
            if last_x == pos.x {
                break;
            }
            last_x = pos.x;
        }
    }
    //  let mut candidates_vel = Vec::new();
    let mut maxy = 0;
    for velx in candidates_velx {
        let mut vely = 1;
        let mut vely_maxy = 0;

        'vely_loop: loop {
            if -vely < target.y.lo {
                break 'vely_loop;
            }
            let mut vel = Velocity { x: velx, y: vely };
            let mut pos = Position { x: 0, y: 0 };
            println!("{velx} {vely} {:?}", pos);
            loop {
                if pos.x >= target.x.lo
                    && pos.x <= target.x.hi
                    && pos.y >= target.y.lo
                    && pos.y <= target.y.hi
                {
                    println!("hit {velx} {vely} {:?}", pos);
                    maxy = if vely_maxy > maxy { vely_maxy } else { maxy };
                    break;
                }
                if pos.y > target.y.hi && pos.x > target.x.hi {
                    break 'vely_loop;
                }
                if pos.y < target.y.lo {
                    break;
                }
                if pos.x > target.x.hi {
                    break;
                }
                vely_maxy = if pos.y > vely_maxy { pos.y } else { vely_maxy };

                step(&mut vel, &mut pos);
            }
            vely += 1;
        }
    }
    println!("{maxy}");
}
