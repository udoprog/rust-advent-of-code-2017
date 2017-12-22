use std::io::{Read, BufRead, BufReader};
use failure::Error;
use std::collections::{HashSet, HashMap};

pub fn part1<R: Read>(reader: R, limit: usize) -> Result<usize, Error> {
    let mut lines: Vec<Vec<bool>> = Vec::new();

    for line in BufReader::new(reader).lines() {
        let line = line?;
        lines.push(line.chars().map(|c| c == '#').collect());
    }

    let x_len = (lines.len() / 2) as i64;
    let y_len = (lines.first().expect("no lines").len() / 2) as i64;

    let mut infected: HashSet<(i64, i64)> = HashSet::new();

    for (y, row) in lines.into_iter().enumerate() {
        for (x, c) in row.into_iter().enumerate() {
            if c {
                infected.insert((x as i64 - x_len, y as i64 - y_len));
            }
        }
    }

    let mut count = 0;

    let mut p: (i64, i64) = (0, 0);
    let mut d: (i64, i64) = (0, -1);

    for _ in 0..limit {
        if infected.contains(&p) {
            // right
            d = (-d.1, d.0);
            infected.remove(&p);
        } else {
            // left
            d = (d.1, -d.0);
            infected.insert(p.clone());
            count += 1;
        }

        p.0 += d.0;
        p.1 += d.1;
    }

    Ok(count)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

pub fn part2<R: Read>(reader: R, limit: usize) -> Result<usize, Error> {
    use self::State::*;

    let mut lines: Vec<Vec<bool>> = Vec::new();

    for line in BufReader::new(reader).lines() {
        let line = line?;
        lines.push(line.chars().map(|c| c == '#').collect());
    }

    let x_len = (lines.len() / 2) as i64;
    let y_len = (lines.first().expect("no lines").len() / 2) as i64;

    let mut states: HashMap<(i64, i64), State> = HashMap::new();

    for (y, row) in lines.into_iter().enumerate() {
        for (x, c) in row.into_iter().enumerate() {
            if c {
                states.insert((x as i64 - x_len, y as i64 - y_len), Infected);
            }
        }
    }

    let mut count = 0;

    let mut p: (i64, i64) = (0, 0);
    let mut d: (i64, i64) = (0, -1);

    for _ in 0..limit {
        let state = states.get(&p).cloned().unwrap_or(Clean);

        let next = match state {
            Clean => {
                // left
                d = (d.1, -d.0);
                Some(Weakened)
            }
            Weakened => Some(Infected),
            Infected => {
                // right
                d = (-d.1, d.0);
                Some(Flagged)
            }
            Flagged => {
                d = (-d.0, -d.1);
                None
            }
        };

        if let Some(next) = next {
            if next == Infected {
                count += 1;
            }

            states.insert(p.clone(), next);
        } else {
            states.remove(&p);
        }

        p.0 += d.0;
        p.1 += d.1;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static INPUT: &str = include_str!("../input/day22.txt");

    #[test]
    fn test_example1() {
        assert_eq!(part1(Cursor::new("..#\n#..\n..."), 10000).unwrap(), 5587);
    }

    #[test]
    fn test_example1_part2() {
        assert_eq!(
            part2(Cursor::new("..#\n#..\n..."), 10_000_000).unwrap(),
            2511944
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(Cursor::new(INPUT), 10000).unwrap(), 5575);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(Cursor::new(INPUT), 10_000_000).unwrap(), 2511991);
    }
}
