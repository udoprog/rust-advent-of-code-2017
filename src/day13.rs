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

pub fn part2<R: Read>(mut reader: R) -> Result<usize, Error> {
    let mut layers = Vec::new();

    for line in BufReader::new(reader).lines() {
        let line = line?;
        let mut line = line.trim().split(":").map(str::trim);

        let id: usize = line.next().expect("left").parse()?;
        let weight: usize = line.next().expect("right").parse()?;

        layers.push((id, weight));
    }

    'outer: for delay in 0.. {
        for &(p, w) in &layers {
            match w {
                0 | 1 => panic!("bad weight: {}", w),
                n => {
                    let adj = 2 * w - 2;

                    if (p + delay + adj) % adj != 0 {
                        continue;
                    }
                }
            }

            continue 'outer;
        }

        return Ok(delay);
    }

    Err(format_err!("no solution found"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn example() {
        assert_eq!(part1(Cursor::new("0: 3\n1: 2\n4: 4\n6: 4\n")).unwrap(), 24);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(Cursor::new("0: 3\n1: 2\n4: 4\n6: 4\n")).unwrap(), 10);
    }
}

const INPUT: &str = include_str!("../input/day13.txt");

problem!{
    tests => [
        run_part1 => {part1(::std::io::Cursor::new(INPUT)), "2de0f8993a4dca954b00b57ee7f9f18e97dbb94cc830107771bc93065177e254"},
        run_part2 => {part2(::std::io::Cursor::new(INPUT)), "e742867a0826f207afc3e3773920b3442daf307629507fc2c5383e44bc0d3771"},
    ];
}
