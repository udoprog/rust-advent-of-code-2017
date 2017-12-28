use std::io::Read;
use failure::Error;
use utils::char_to_digit;

pub fn run<O, R: Read>(mut reader: R, offset: O) -> Result<u32, Error>
where
    O: Fn(usize) -> usize,
{
    let mut data = String::new();
    reader.read_to_string(&mut data)?;

    let dig: Vec<u32> = data.trim()
        .chars()
        .map(char_to_digit)
        .collect::<Result<Vec<_>, Error>>()?;

    let o = offset(dig.len());

    let sum = dig.iter()
        .cloned()
        .enumerate()
        .map(|(i, a)| (a, dig[(i + o) % dig.len()]))
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| a)
        .sum();

    Ok(sum)
}

const INPUT: &str = include_str!("../input/day1.txt");

problem!{
    tests => [
        part1 => {run(::std::io::Cursor::new(INPUT), |_| 1), "6e084fb086c0433c7b59668741b1984898fb8668b00a9f10f8407709de64c4de"},
        part2 => {run(::std::io::Cursor::new(INPUT), |size| size / 2), "87d8a19ca3abd85d2635b2cd83634bfac62dec00950770ee4bb2cc3914524906"},
    ];
}
