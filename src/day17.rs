use failure::Error;
use std::mem;

pub fn part1(step: usize, limit: usize) -> Result<usize, Error> {
    let mut buffer = vec![0usize];

    let mut c = 0usize;

    for i in 1usize..=limit {
        c = (c + step) % buffer.len();
        c = c + 1;
        buffer.insert(c, i);
    }

    Ok(buffer[(c + 1) % buffer.len()])
}

pub fn part2(step: usize, limit: usize) -> Result<Option<usize>, Error> {
    let mut second = None;

    let mut c = 0usize;

    for i in 1usize..=limit {
        c = (c + step) % i;

        if c == 0 {
            mem::replace(&mut second, Some(i));
        }

        c = c + 1;
    }

    Ok(second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(part1(3, 2017).unwrap(), 638);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(348, 2017).unwrap(), 417);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(348, 50_000_000).unwrap(), Some(34334221));
    }
}
