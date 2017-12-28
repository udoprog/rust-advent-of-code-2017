use std::io::Read;
use failure::Error;
use std::collections::HashMap;
use std::mem;

pub fn index(index: u64, _: u64) -> u64 {
    index
}

pub fn distance(index: u64, prev: u64) -> u64 {
    index - prev
}

pub fn run<R: Read, A>(mut reader: R, accessor: A) -> Result<u64, Error>
where
    A: Fn(u64, u64) -> u64,
{
    let mut data = String::new();
    reader.read_to_string(&mut data)?;

    let mut banks = data.trim()
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<u32>, _>>()?;

    let mut states: HashMap<Vec<u32>, usize> = HashMap::new();
    let size = banks.len();

    states.insert(banks.clone(), 0);

    for i in 1usize.. {
        let idx = banks
            .iter()
            .max()
            .and_then(|max| banks.iter().enumerate().find(|v| *v.1 == *max))
            .expect("a max value")
            .0;

        let value = mem::replace(&mut banks[idx], 0u32) as usize;

        for i in idx + 1..idx + value + 1 {
            banks[i % size] += 1;
        }

        if let Some(prev) = states.insert(banks.clone(), i) {
            return Ok(accessor(i as u64, prev as u64));
        }
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom() {
        assert_eq!(5, run(::std::io::Cursor::new("0\t2\t7\t0"), index).unwrap());
    }
}

const INPUT: &str = include_str!("../input/day6.txt");

problem!{
    tests => [
        part1 => {run(::std::io::Cursor::new(INPUT), index), "e57d2ad0746ead65c57193eaad6d62713cad20f086f0c205f72fbc274011be2d"},
        part2 => {run(::std::io::Cursor::new(INPUT), distance), "08d8be2e105c68cca0598a301cb51758333bf290d438fc8bff62f03dab44eebe"},
    ];
}
