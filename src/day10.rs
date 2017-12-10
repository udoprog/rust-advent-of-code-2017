use failure::Error;
use std::fmt;
use std::io::Read;

pub struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    pub fn new<T>(data: &'a T) -> HexSlice<'a>
    where
        T: ?Sized + AsRef<[u8]> + 'a,
    {
        HexSlice(data.as_ref())
    }
}

impl<'a> fmt::Display for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

impl<'a> fmt::Debug for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

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

pub fn part2<R: Read>(mut reader: R) -> Result<String, Error> {
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

    Ok(HexSlice::new(&out).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static INPUT: &str = include_str!("../input/day10.txt");

    #[test]
    fn test_simple() {
        assert_eq!(part1(Cursor::new("3,4,1,5"), 4).unwrap(), 12);
    }

    #[test]
    fn test_all() {
        assert_eq!(part1(Cursor::new(INPUT), 255).unwrap(), 826);
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
    fn p2() {
        assert_eq!(
            part2(Cursor::new(INPUT)).unwrap().as_str(),
            "d067d3f14d07e09c2e7308c3926605c4"
        );
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
