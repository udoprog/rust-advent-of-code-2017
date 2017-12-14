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
    use std::io::Cursor;

    static DAY4: &str = include_str!("../input/day4.txt");

    #[test]
    fn test_unique() {
        let mut input = Cursor::new(DAY4);
        assert_eq!(451, run(&mut input, unique).unwrap());
    }

    #[test]
    fn test_anagram_example() {
        let mut input = Cursor::new(&"oiii ioii iioi iiio\niiii oiii ooii oooi oooo"[..]);
        assert_eq!(1, run(&mut input, unique_anagrams).unwrap());
    }

    #[test]
    fn test_anagram() {
        let mut input = Cursor::new(DAY4);
        assert_eq!(223, run(&mut input, unique_anagrams).unwrap());
    }
}
