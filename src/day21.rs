use std::io::{Read, BufRead, BufReader};
use failure::Error;
use std::fmt;
use std::collections::{HashSet, HashMap};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Image {
    data: Vec<bool>,
    size: usize,
}

impl fmt::Debug for Image {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for line in self.data.chunks(self.size) {
            for b in line {
                if *b {
                    write!(fmt, "#")?;
                } else {
                    write!(fmt, ".")?;
                }
            }

            write!(fmt, "/")?;
        }

        Ok(())
    }
}

impl Image {
    pub fn new(size: usize) -> Image {
        Image {
            data: vec![false; size * size],
            size: size,
        }
    }

    fn offset(&self, x: usize, y: usize) -> usize {
        x + y * self.size
    }

    pub fn merge(&mut self, x: usize, y: usize, image: &Image) {
        for (sx, dx) in (x..(x + image.size)).enumerate() {
            for (sy, dy) in (y..(y + image.size)).enumerate() {
                let v = image.get(sx, sy).expect("no pixel");
                self.set(dx, dy, v);
            }
        }
    }

    pub fn set_row(&mut self, y: usize, values: &[bool]) {
        let offset = y * self.size;
        let end = offset + self.size;

        if offset > self.data.len() {
            return;
        }

        for (d, s) in self.data[offset..end].iter_mut().zip(
            values.iter().cloned(),
        )
        {
            *d = s;
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        let o = self.offset(x, y);
        *self.data.get_mut(o).expect("bad offset") = value;
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.data.get(x + y * self.size).cloned()
    }

    pub fn flip(&self) -> Image {
        let mut out = self.clone();

        for y in 0..out.size {
            for (x0, x1) in (0..(out.size / 2)).zip(((out.size / 2)..out.size).rev()) {
                let o0 = out.offset(x0, y);
                let o1 = out.offset(x1, y);
                out.data.swap(o0, o1);
            }
        }

        out
    }

    pub fn rotate(&self) -> Image {
        let mut out = self.clone();

        for row in 0..(out.size / 2) {
            let end = row + out.size - row * 2;

            for (x, y) in (row..end).zip(row..end) {
                let v = self.get(x, row).expect("no pixel");
                out.set(self.size - row - 1, y, v);

                let v = self.get(self.size - row - 1, y).expect("no pixel");
                out.set(self.size - x - 1, self.size - row - 1, v);

                let v = self.get(self.size - x - 1, self.size - row - 1).expect("no pixel");
                out.set(row, self.size - y - 1, v);

                let v = self.get(row, self.size - y - 1).expect("no pixel");
                out.set(x, row, v);
            }
        }

        out
    }

    pub fn chunks(&self, size: usize) -> Chunks {
        Chunks {
            source: self,
            size: size,
            x: 0usize,
            y: 0usize,
        }
    }
}

pub struct Chunks<'a> {
    source: &'a Image,
    size: usize,
    x: usize,
    y: usize,
}

impl<'a> Iterator for Chunks<'a> {
    type Item = (usize, usize, Image);

    fn next(&mut self) -> Option<Self::Item> {
        if (self.x * self.size) >= self.source.size {
            self.x = 0;
            self.y += 1;
        }

        if (self.y * self.size) >= self.source.size {
            return None;
        }

        let start_x = self.x * self.size;
        let start_y = self.y * self.size;

        let mut out = Image::new(self.size);

        for (dx, sx) in (start_x..(start_x + self.size)).enumerate() {
            for (dy, sy) in (start_y..(start_y + self.size)).enumerate() {
                let v = self.source.get(sx, sy).expect("no pixel");
                out.set(dx, dy, v);
            }
        }

        let x = self.x;
        let y = self.y;

        self.x += 1;
        Some((x, y, out))
    }
}

pub fn run<R: Read>(reader: R, limit: usize) -> Result<usize, Error> {
    let mut patterns = HashMap::new();

    for line in BufReader::new(reader).lines() {
        let line = line?;
        let mut it = line.split("=>");
        let pat = image_from(it.next().expect("no pattern"));
        let dest = image_from(it.next().expect("no dest"));

        let mut unique = HashSet::new();
        let mut current = pat.clone();

        for _ in 0..4 {
            unique.insert(current.clone());
            current = current.rotate();
        }

        unique.insert(current.clone());

        let mut current = pat.flip();

        for _ in 0..4 {
            unique.insert(current.clone());
            current = current.rotate();
        }

        unique.insert(current.clone());

        for pat in unique {
            if patterns.insert(pat, dest.clone()).is_some() {
                panic!("pattern already present");
            }
        }
    }

    let mut image = Image::new(3);

    image.set_row(0, &[false, true, false]);
    image.set_row(1, &[false, false, true]);
    image.set_row(2, &[true, true, true]);

    for _ in 0..limit {
        if image.size % 2 == 0 {
            let mut new_image = Image::new(image.size + image.size / 2);

            for (x, y, img) in image.chunks(2) {
                let m = patterns.get(&img).expect("no match");
                new_image.merge(x * m.size, y * m.size, m);
            }

            image = new_image;
            continue;
        }

        if image.size % 3 == 0 {
            let mut new_image = Image::new(image.size + image.size / 3);

            for (x, y, img) in image.chunks(3) {
                let m = patterns.get(&img).expect("no match");
                new_image.merge(x * m.size, y * m.size, m);
            }

            image = new_image;
            continue;
        }

        panic!("oh no, bad image!");
    }

    return Ok(image.data.into_iter().filter(|d| *d).count());

    fn image_from(input: &str) -> Image {
        let data: Vec<Vec<bool>> = input
            .trim()
            .split('/')
            .map(|p| p.chars().map(|c| c == '#').collect())
            .collect();

        let size = data.first().expect("at least one").len();

        let mut image = Image::new(size);

        for (y, line) in data.into_iter().enumerate() {
            image.set_row(y, &line);
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_example() {
        assert_eq!(
            run(
                Cursor::new("../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#"),
                2,
            ).unwrap(),
            12
        );
    }

    #[test]
    fn test_image() {
        let mut a = Image::new(4);

        a.set_row(0, &[true, false, false, false]);
        a.set_row(1, &[true, false, true, true]);
        a.set_row(2, &[true, true, false, true]);
        a.set_row(3, &[false, true, true, true]);

        let mut b = Image::new(4);

        b.set_row(0, &[false, true, true, true]);
        b.set_row(1, &[true, true, false, false]);
        b.set_row(2, &[true, false, true, false]);
        b.set_row(3, &[true, true, true, false]);

        assert_eq!(a.rotate(), b);
        assert_eq!(a.rotate().rotate().rotate().rotate(), a);
    }

    #[test]
    fn test_merge() {
        let mut a = Image::new(4);

        a.set_row(0, &[true, false, false, false]);
        a.set_row(1, &[true, true, true, true]);
        a.set_row(2, &[true, true, true, true]);
        a.set_row(3, &[false, true, true, true]);

        let mut b = Image::new(2);

        b.set_row(0, &[true, true]);
        b.set_row(1, &[true, true]);

        let mut c = Image::new(4);

        c.set_row(0, &[true, false, false, false]);
        c.set_row(1, &[true, true, true, true]);
        c.set_row(2, &[true, true, true, true]);
        c.set_row(3, &[false, true, true, true]);

        a.merge(1, 1, &b);

        assert_eq!(a, c);
    }
}

const INPUT: &str = include_str!("../input/day21.txt");

problem!{
    tests => [
        run_part1 => {run(::std::io::Cursor::new(INPUT), 5), "b86fc9ae92b1d6b53dddd0017666021afdc0643bd03defbb0c8918495d79cfd3"},
        run_part2 => {run(::std::io::Cursor::new(INPUT), 18), "d13ebb233195570a27ffe3936212b6a7f02b0ef7d040dd85561b929805ed64f3"},
    ];
}
