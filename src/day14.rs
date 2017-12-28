use std::io::Cursor;
use std::collections::{HashSet, VecDeque};
use failure::Error;
use bit_vec::BitVec;

use day10::hash;

pub fn part1(input: &str) -> Result<usize, Error> {
    let mut squares = 0;

    for line in 0..128 {
        let out = hash(Cursor::new(format!("{}-{}", input, line)))?;

        for mut b in out {
            while b > 0 {
                if b % 2 == 1 {
                    squares += 1;
                }

                b = b >> 1;
            }
        }
    }

    Ok(squares)
}

pub fn part2(input: &str) -> Result<usize, Error> {
    let mut grid: HashSet<(i32, i32)> = HashSet::new();

    for y in 0..128 {
        let bytes = hash(Cursor::new(format!("{}-{}", input, y)))?;
        let bits = BitVec::from_bytes(&bytes);
        grid.extend(bits.into_iter().enumerate().filter(|v| v.1).map(|v| {
            (v.0 as i32, y as i32)
        }))
    }

    let mut regions = 0;
    let mut queue = VecDeque::new();

    loop {
        let k = {
            if let Some(k) = grid.iter().next() {
                *k
            } else {
                break;
            }
        };

        grid.remove(&k);

        regions += 1;
        queue.push_back(k);

        while let Some((x, y)) = queue.pop_front() {
            queue.extend(grid.take(&(x - 1, y)));
            queue.extend(grid.take(&(x, y - 1)));
            queue.extend(grid.take(&(x + 1, y)));
            queue.extend(grid.take(&(x, y + 1)));
        }
    }

    Ok(regions)
}

problem!{
    tests => [
        run_part1 => {part1("ljoxqyyw"), "bdcdda0fde396f250dee097af986dd4e60386ede1d07a8911538d4f43f6d5808"},
        run_part2 => {part2("ljoxqyyw"), "b77146fa1070eca02cb4071195b134a6eb3e69a3b7047f97c74112cac76cf0ce"},
    ];
}
