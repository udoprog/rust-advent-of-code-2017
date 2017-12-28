use std::io::{Read, BufReader, BufRead};
use std::ops::{GeneratorState, Generator};

use failure::Error;

fn numgen(
    mut num: u64,
    factor: u64,
    modulo: u64,
    check_factor: u64,
) -> impl Generator<Yield = u64, Return = !> {
    move || loop {
        num = (num * factor) % modulo;

        if num % check_factor == 0 {
            yield num;
        }
    }
}

pub fn run(a: u64, b: u64, upper: usize, a_factor: u64, b_factor: u64) -> usize {
    let mut count = 0;

    let mask = 0b1111_1111_1111_1111;
    let modulo = 2147483647;

    let mut a_gen = numgen(a, 16807, modulo, a_factor);
    let mut b_gen = numgen(b, 48271, modulo, b_factor);

    for _ in 0..upper {
        let GeneratorState::Yielded(a) = a_gen.resume();
        let GeneratorState::Yielded(b) = b_gen.resume();

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

const INPUT: &str = include_str!("../input/day15.txt");

problem!{
    tests => [
        run_part1 => {part1(::std::io::Cursor::new(INPUT)), "832205b1b70f68d957f7cd606f4f127a3d866a843123ec3ef16f851a146b6d6e"},
        run_part2 => {part2(::std::io::Cursor::new(INPUT)), "8f23ac526a26a8acce625bf3d571700c15df4c92e4d60dc8ffbd8c0fa4286487"},
    ];
}
