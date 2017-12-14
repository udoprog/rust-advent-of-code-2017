#![allow(unused)]

use std::io::{BufRead, BufReader, Read};
use std::collections::{HashMap, HashSet, VecDeque};
use failure::Error;

pub fn part1<R: Read>(mut reader: R) -> Result<usize, Error> {
    let mut layers = Vec::new();

    for line in BufReader::new(reader).lines() {
        let line = line?;
        let mut line = line.trim().split(":").map(str::trim);

        let id: usize = line.next().expect("left").parse()?;
        let weight: usize = line.next().expect("right").parse()?;

        layers.push((id, weight));
    }

    let mut severity = 0;

    for &(p, w) in &layers {
        match w {
            0 | 1 => panic!("bad weight: {}", w),
            n => {
                let adjusted = 2 * w - 2;

                if (p + adjusted) % adjusted != 0 {
                    continue;
                }
            }
        }

        severity += p * w;
    }

    Ok(severity)
}


fn step(layers: &mut HashMap<i64, (i64, i64, i64)>) {
    // simulate layers
    for (id, value) in layers.iter_mut() {
        let current = &mut value.0;
        let weight = &mut value.1;
        let dir = &mut value.2;

        *current += *dir;

        match *dir {
            -1 => {
                if *current == 0 {
                    *dir = -*dir;
                }
            }
            1 => {
                if *current == (*weight - 1) {
                    *dir = -*dir;
                }
            }
            _ => {
            }
        }
    }
}

pub fn part2_brute<R: Read>(mut reader: R) -> Result<usize, Error> {
    let mut layers: HashMap<i64, (i64, i64, i64)> = HashMap::new();

    let mut upper = 0i64;

    for line in BufReader::new(reader).lines() {
        let line = line?;
        let mut line = line.trim().split(":").map(str::trim);

        let id: i64 = line.next().expect("left").parse()?;
        let weight: i64 = line.next().expect("right").parse()?;

        layers.insert(id, (0, weight, 1));
        upper = i64::max(upper, id);
    }

    'outer: for delay in 0.. {
        // reset
        for (id, value) in layers.iter_mut() {
            value.0 = 0;
            value.2 = 1;
        }

        println!("testing = {}", delay);

        for _ in 0..delay {
            step(&mut layers);
        }

        for p in 0..=upper {
            if let Some(&(current, weight, _)) = layers.get(&p) {
                if current == 0 {
                    continue 'outer;
                }
            }

            step(&mut layers);
        }

        return Ok(delay);
    }

    Err(format_err!("no good delay found"))
}

pub fn part2<R: Read>(mut reader: R) -> Result<usize, Error> {
    let mut layers = Vec::new();

    for line in BufReader::new(reader).lines() {
        let line = line?;
        let mut line = line.trim().split(":").map(str::trim);

        let id: usize = line.next().expect("left").parse()?;
        let weight: usize = line.next().expect("right").parse()?;

        layers.push((id, weight));
    }

    let mut severity = 0;

    for delay in 0.. {
        println!("delay = {}", delay);

        for &(p, w) in &layers {
            match w {
                0 | 1 => panic!("bad weight: {}", w),
                n => {
                    let adj = 2 * w - 2;

                    if (p + adj) % adj != 0 {
                        continue;
                    }
                }
            }

            severity += p * w;
        }

        if severity == 0 {
            return Ok(delay);
        }
    }

    Ok(severity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static INPUT: &str = include_str!("../input/day13.txt");

    #[test]
    fn example() {
        assert_eq!(part1(Cursor::new("0: 3\n1: 2\n4: 4\n6: 4\n")).unwrap(), 24);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(Cursor::new("0: 3\n1: 2\n4: 4\n6: 4\n")).unwrap(), 2);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(Cursor::new(INPUT)).unwrap(), 1632);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(Cursor::new(INPUT)).unwrap(), 330024);
    }
}
