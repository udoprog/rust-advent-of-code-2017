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

const INPUT: &str = include_str!("../input/day5.txt");

problem!{
    tests => [
        part1 => {run(::std::io::Cursor::new(INPUT), |v| v + 1), "7abbc3cc7445b32c47f65d826b20291cd185ddc162faf02d59da6739c1f841ea"},
        part2 => {run(::std::io::Cursor::new(INPUT), |v| if v < 3 { v + 1 } else { v - 1 }), "404c62038bb1c873e58c108f154de9e1d5581923d03f7bae4e8e3a5772173224"},
    ];
}
