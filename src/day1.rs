use std::fs::File;
use std::io::Read;
use failure::Error;
use std::path::Path;
use utils::char_to_digit;

pub fn run<O, P: AsRef<Path>>(path: P, offset: O) -> Result<u32, Error>
where
    O: Fn(usize) -> usize,
{
    let mut data = String::new();
    File::open(path)?.read_to_string(&mut data)?;

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
