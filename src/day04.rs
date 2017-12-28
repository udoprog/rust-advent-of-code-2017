use std::io::{BufReader, BufRead, Read};
use failure::Error;
use std::collections::HashSet;

/// Reduce the inputs to the number o funique elements.
pub fn unique(input: &[&str]) -> usize {
    input.iter().cloned().collect::<HashSet<_>>().len()
}

/// Reduce the inputs to the number of elements which are unique when read as an anagram.
pub fn unique_anagrams(input: &[&str]) -> usize {
    input
        .iter()
        .cloned()
        .map(|s| {
            let mut im = s.chars().collect::<Vec<char>>();
            im.sort();
            im
        })
        .collect::<HashSet<_>>()
        .len()
}

pub fn run<C, R: Read>(reader: R, calc: C) -> Result<u32, Error>
where
    C: Fn(&[&str]) -> usize,
{
    let mut r = BufReader::new(reader);
    let mut data = String::new();
    let mut sum = 0u32;

    while r.read_line(&mut data)? > 0 {
        {
            let line: Vec<&str> = data.split(' ').map(str::trim).collect();

            if calc(&line) == line.len() {
                sum += 1;
            }
        }

        data.clear();
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_example() {
        let mut input = ::std::io::Cursor::new(&"oiii ioii iioi iiio\niiii oiii ooii oooi oooo"[..]);
        assert_eq!(1, run(&mut input, unique_anagrams).unwrap());
    }
}

const INPUT: &str = include_str!("../input/day4.txt");

problem!{
    tests => [
        part1 => {run(::std::io::Cursor::new(INPUT), unique), "e2dbec6555bb101fae22904565c04818e98686e2c3b60164a576f9e79a24168f"},
        part2 => {run(::std::io::Cursor::new(INPUT), unique_anagrams), "26530224e366822080badeafd24c466f80dc067fca2d79fe1b6a38965e71f3c7"},
    ];
}
