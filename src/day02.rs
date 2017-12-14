use std::io::{BufReader, BufRead, Read};
use failure::Error;

pub fn minmax(values: &[u32]) -> u32 {
    if let Some(first) = values.first() {
        let (mn, mx) = values.iter().skip(1).fold((*first, *first), |(mn, mx), v| {
            (u32::min(mn, *v), u32::max(mx, *v))
        });

        mx - mn
    } else {
        0
    }
}

pub fn evendiv(values: &[u32]) -> u32 {
    for (i, v) in values.iter().cloned().enumerate() {
        for o in &values[i + 1..] {
            if v % o == 0 {
                return v / o;
            }

            if o % v == 0 {
                return o / v;
            }
        }
    }

    0
}

pub fn run<C, R: Read>(reader: R, calc: C) -> Result<u32, Error>
where
    C: Fn(&[u32]) -> u32,
{
    let mut r = BufReader::new(reader);
    let mut data = String::new();

    let mut sum = 0u32;

    while r.read_line(&mut data)? > 0 {
        let line: Vec<u32> = data.split('\t')
            .map(str::trim)
            .map(|s| s.parse().map_err(Into::into))
            .collect::<Result<Vec<_>, Error>>()?;

        sum += calc(&line);
        data.clear();
    }

    Ok(sum)
}
