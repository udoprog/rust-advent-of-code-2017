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
}

problem!{
    tests => [
        run_part1 => {part1(348, 2017), "a22202bc74d2bd50711ec9752d3ca91c30b151badc2473860ec6cc358d6bf797"},
        run_part2 => {part2(348, 50_000_000), "be6817723645dca56b8a07acb273d78ad89b9ae63e9d92028a46e00a64725d5a"},
    ];
}
