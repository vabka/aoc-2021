use text_io::read;

const FORWARD: &str = "forward";
const DOWN: &str = "down";
const UP: &str = "up";

enum Direction {
    Forward,
    Down,
    Up,
}

struct Move {
    dir: Direction,
    value: u32,
}
impl Direction {
    fn parse(txt: &str) -> Option<Direction> {
        match txt.trim() {
            FORWARD => Some(Direction::Forward),
            DOWN => Some(Direction::Down),
            UP => Some(Direction::Up),
            _ => None,
        }
    }
}
impl Move {
    fn new(dir: Direction, value: u32) -> Move {
        Move { dir, value }
    }

    fn parse(text: &str) -> Option<Move> {
        if let Some((fst, snd)) = text.split_once(' ') {
            let dir = Direction::parse(fst)?;
            let value = snd.trim().parse::<u32>();
            match value {
                Ok(v) => Some(Move::new(dir, v)),
                _ => None,
            }
        } else {
            None
        }
    }
}

pub fn solve() {
    solve_pt2();
}
fn solve_pt1() {
    let mut horizontal_position = 0;
    let mut depth = 0;
    loop {
        let line: String = read!("{}\n");
        if let Some(movement) = Move::parse(&line) {
            match movement.dir {
                Direction::Forward => horizontal_position += movement.value,
                Direction::Down => depth += movement.value,
                Direction::Up => depth -= movement.value,
            }
        } else {
            break;
        }
    }
    println!("{}", horizontal_position * depth)
}
fn solve_pt2() {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    loop {
        let line: String = read!("{}\n");
        if let Some(movement) = Move::parse(&line) {
            match movement.dir {
                Direction::Forward => {
                    horizontal_position += movement.value;
                    depth += movement.value * aim;
                },
                Direction::Down => aim += movement.value,
                Direction::Up => aim -= movement.value,
            }
        } else {
            break;
        }
    }
    println!("{}", horizontal_position * depth)
}