use std::io::{BufRead, BufReader, Read};
use failure::Error;

pub fn run<R: Read, J>(reader: R, jump: J) -> Result<u32, Error>
where
    J: Fn(i32) -> i32,
{
    let mut jumps = BufReader::new(reader)
        .lines()
        .map(|l| {
            l.map_err(Into::into).and_then(|l| {
                l.trim().parse::<i32>().map_err(Into::into)
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let mut p = 0i64;

    for s in 0usize.. {
        if p < 0 {
            return Ok(s as u32);
        }

        match jumps.get_mut(p as usize) {
            Some(value) => {
                p += *value as i64;
                *value = jump(*value);
            }
            None => return Ok(s as u32),
        }
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static DAY5: &str = include_str!("../input/day5.txt");

    #[test]
    fn test_one() {
        assert_eq!(355965, run(Cursor::new(DAY5), |v| v + 1).unwrap());
    }
}
