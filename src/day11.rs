use failure::Error;
use std::io::Read;

use self::Step::*;

#[derive(Debug)]
enum Step {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

impl Step {
    fn step(&self) -> (i32, i32, i32) {
        match *self {
            N => (1, 0, -1),
            NE => (1, -1, 0),
            SE => (0, -1, 1),
            S => (-1, 0, 1),
            SW => (-1, 1, 0),
            NW => (0, 1, -1),
        }
    }
}

fn parse_step(input: &str) -> Option<Step> {
    let out = match input {
        "n" => N,
        "ne" => NE,
        "se" => SE,
        "s" => S,
        "sw" => SW,
        "nw" => NW,
        _ => return None,
    };

    Some(out)
}

fn distance(p: (i32, i32, i32)) -> u32 {
    let (x, y, z) = p;
    ((x.abs() + y.abs() + z.abs()) / 2) as u32
}

pub fn run<R: Read>(mut reader: R) -> Result<(u32, u32), Error> {
    let mut data = String::new();
    reader.read_to_string(&mut data)?;

    let steps: Vec<Step> = data.trim()
        .split(',')
        .map(parse_step)
        .flat_map(|v| v)
        .collect();

    let mut p = (0i32, 0i32, 0i32);
    let mut max = 0u32;

    for step in steps {
        let s = step.step();
        p = (p.0 + s.0, p.1 + s.1, p.2 + s.2);
        max = u32::max(max, distance(p));
    }

    Ok((distance(p), max))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static INPUT: &str = include_str!("../input/day11.txt");

    #[test]
    fn test_problem() {
        assert_eq!(run(Cursor::new(INPUT)).unwrap(), (685, 1457));
    }
}
