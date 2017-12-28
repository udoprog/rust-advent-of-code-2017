use failure::Error;
use std::io::Read;
use hex_slice::HexSlice;

fn reverse<T>(d: &mut [T], pos: usize, length: usize) {
    let len = d.len();

    for (a, b) in (0..length / 2).zip((0..length).rev()) {
        d.swap((pos + a) % len, (pos + b) % len);
    }
}

pub fn part1<R: Read>(mut reader: R, end: usize) -> Result<usize, Error> {
    let mut line = String::new();
    reader.read_to_string(&mut line)?;

    let lengths: Vec<usize> = line.trim()
        .split(',')
        .map(|d| d.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut numbers: Vec<usize> = (0usize..=end).collect();

    let mut pos = 0usize;

    for (skip, length) in lengths.into_iter().enumerate() {
        reverse(&mut numbers, pos, length);
        pos = (pos + length + skip) % numbers.len();
    }

    Ok(numbers[0] * numbers[1])
}

pub fn hash<R: Read>(mut reader: R) -> Result<Vec<u8>, Error> {
    let mut line = String::new();
    reader.read_to_string(&mut line)?;
    let line = line.trim();

    let lengths: Vec<usize> = line.as_bytes()
        .iter()
        .chain(&[17, 31, 73, 47, 23])
        .map(|l| *l as usize)
        .collect();

    let mut sparse: Vec<u8> = (0..=255).collect();

    let mut pos = 0usize;
    let mut skip = 0usize..;

    for _ in 0..64 {
        for (l, skip) in lengths.iter().zip(&mut skip) {
            reverse(&mut sparse, pos, *l);
            pos = (pos + skip + *l) % sparse.len();
        }
    }

    let out: Vec<u8> = sparse
        .chunks(16)
        .map(|chunk| chunk.into_iter().fold(0u8, |s, v| s ^ v))
        .collect();

    Ok(out)
}

pub fn part2<R: Read>(reader: R) -> Result<String, Error> {
    let out = hash(reader)?;
    Ok(HexSlice::new(&out).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_simple() {
        assert_eq!(part1(Cursor::new("3,4,1,5"), 4).unwrap(), 12);
    }

    #[test]
    fn test_reverse() {
        let mut d = vec![1, 2, 3, 4, 5, 6];
        reverse(&mut d, 0, 6);
        assert_eq!(vec![6, 5, 4, 3, 2, 1], d);

        let mut d = vec![1, 2, 3, 4, 5, 6];
        reverse(&mut d, 1, 6);
        assert_eq!(vec![2, 1, 6, 5, 4, 3], d);
    }

    #[test]
    fn p2_examples() {
        assert_eq!(
            part2(Cursor::new("")).unwrap().as_str(),
            "a2582a3a0e66e6e86e3812dcb672a272"
        );
        assert_eq!(
            part2(Cursor::new("AoC 2017")).unwrap().as_str(),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
        assert_eq!(
            part2(Cursor::new("1,2,3")).unwrap().as_str(),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );
        assert_eq!(
            part2(Cursor::new("1,2,4")).unwrap().as_str(),
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }
}

const INPUT: &str = include_str!("../input/day10.txt");

problem!{
    tests => [
        run_part1 => {part1(::std::io::Cursor::new(INPUT), 255), "6f6e675d6f712b625eb01d5ad19ef2b10f628f870a8006df936cc4b942862459"},
        run_part2 => {part2(::std::io::Cursor::new(INPUT)), "575f20763a3442b392e95c10cb98eb784192caeb07543297a3f01feef60b3b2d"},
    ];
}
