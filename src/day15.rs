use std::io::{Read, BufReader, BufRead};

use failure::Error;

pub fn run(mut a: u64, mut b: u64, upper: usize, a_factor: u64, b_factor: u64) -> usize {
    let mut count = 0;

    let mask = 0b1111_1111_1111_1111;
    let modulo = 2147483647;

    for _ in 0..upper {
        a = (a * 16807) % modulo;

        while a % a_factor != 0 {
            a = (a * 16807) % modulo;
        }

        b = (b * 48271) % modulo;

        while b % b_factor != 0 {
            b = (b * 48271) % modulo;
        }

        if a & mask == b & mask {
            count += 1;
        }
    }

    count
}

fn parse<R: Read>(input: R) -> Result<(u64, u64), Error> {
    let mut lines = BufReader::new(input)
        .lines()
        .map(|line| line.expect("bad line"))
        .flat_map(|line| line.split_whitespace().nth(4).map(ToOwned::to_owned));

    let a: u64 = lines.next().expect("bad a").parse()?;
    let b: u64 = lines.next().expect("bad b").parse()?;

    Ok((a, b))
}

pub fn part1<R: Read>(input: R) -> Result<usize, Error> {
    let (a, b) = parse(input)?;
    Ok(run(a, b, 40_000_000, 1, 1))
}

pub fn part2<R: Read>(input: R) -> Result<usize, Error> {
    let (a, b) = parse(input)?;
    Ok(run(a, b, 5_000_000, 4, 8))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static INPUT: &str = include_str!("../input/day15.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(Cursor::new(INPUT)).unwrap(), 609);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(Cursor::new(INPUT)).unwrap(), 253);
    }
}
