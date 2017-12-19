use std::collections::HashMap;
use std::io::{Read, BufRead, BufReader};

pub fn run<R: Read>(reader: R) -> (String, usize) {
    let mut grid: HashMap<(i64, i64), char> = HashMap::new();
    let mut entry = None;

    for (y, line) in BufReader::new(reader).lines().enumerate() {
        for (x, c) in line.expect("bad line").chars().enumerate() {
            if y == 0 && c == '|' {
                entry = Some((x as i64, y as i64));
            }

            if !c.is_whitespace() {
                grid.insert((x as i64, y as i64), c);
            }
        }
    }

    let mut out = String::new();
    let (mut x, mut y) = entry.expect("no entry");
    let mut d = (0i64, 1i64);
    let mut steps = 0;

    while let Some(c) = grid.get(&(x, y)) {
        match *c {
            '|' | '-' => {}
            '+' => d = pick(&grid, x, y, d).expect("no turn"),
            c => out.push(c),
        }

        x += d.0;
        y += d.1;
        steps += 1;
    }

    return (out, steps);

    fn pick(grid: &HashMap<(i64, i64), char>, x: i64, y: i64, d: (i64, i64)) -> Option<(i64, i64)> {
        for n in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            // do not move backwards.
            if (-n.0, -n.1) == d {
                continue;
            }

            if let Some(c) = grid.get(&(x + n.0, y + n.1)) {
                match *c {
                    '|' | '-' | '+' => return Some(*n),
                    _ => {}
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static INPUT: &str = include_str!("../input/day19.txt");

    #[test]
    fn test_both() {
        assert_eq!(run(Cursor::new(INPUT)), ("MKXOIHZNBL".to_string(), 17872));
    }
}
