use std::io::{Read, BufReader, BufRead};
use std::collections::HashMap;

use failure::Error;

pub enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

fn parse<R: Read>(input: R) -> Result<Vec<Move>, Error> {
    let mut out = Vec::new();

    for line in BufReader::new(input).lines() {
        let line = line?;

        for m in line.split(',') {
            match m.chars().next().expect("first character") {
                's' => {
                    out.push(Move::Spin(m[1..].parse()?));
                }
                'p' => {
                    let mut it = m[1..].split('/');
                    let a = it.next().and_then(|a| a.chars().next()).expect(
                        "missing partner one",
                    );
                    let b = it.next().and_then(|b| b.chars().next()).expect(
                        "missing partner two",
                    );
                    out.push(Move::Partner(a as u8, b as u8));
                }
                'x' => {
                    let mut it = m[1..].split('/');
                    let a = it.next().expect("missing position one").parse()?;
                    let b = it.next().expect("missing position two").parse()?;
                    out.push(Move::Exchange(a, b));
                }
                c => panic!("unexpected op: {}", c),
            }
        }
    }

    Ok(out)
}

pub fn run<R: Read>(input: R, count: u8, limit: usize) -> Result<String, Error> {
    use self::Move::*;

    let dancers = b'a'..('a' as u8 + count);
    let mut dancers: Vec<u8> = dancers.collect();

    let moves = parse(input)?;

    let mut drained = Vec::new();

    let mut known: HashMap<Vec<u8>, usize> = HashMap::new();
    let mut by_index: HashMap<usize, Vec<u8>> = HashMap::new();

    for rep in 0..limit {
        // detect cycle
        if let Some(found) = known.get(&dancers) {
            let cycle = rep - found;
            let index = (limit - rep) % cycle;
            dancers = by_index.get(&index).cloned().expect("missing dancers");
            break;
        }

        known.insert(dancers.clone(), rep);
        by_index.insert(rep, dancers.clone());

        for m in &moves {
            match *m {
                Exchange(a, b) => {
                    dancers.swap(a, b);
                }
                Partner(a, b) => {
                    let pa = dancers.iter().position(|fa| a == *fa).expect("pos a");
                    let pb = dancers.iter().position(|fb| b == *fb).expect("pos b");
                    dancers.swap(pa, pb);
                }
                Spin(n) => {
                    let len = dancers.len();

                    drained.clear();
                    drained.extend(dancers.drain(..(len - n)));
                    dancers.extend(drained.iter().cloned());
                }
            }
        }
    }

    Ok(String::from_utf8(dancers.into_iter().collect())?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_example1() {
        assert_eq!(
            run(Cursor::new("s1,x3/4,pe/b"), 5, 1).unwrap().as_str(),
            "baedc"
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            run(Cursor::new("s1,x3/4,pe/b"), 5, 2).unwrap().as_str(),
            "ceadb"
        );
    }
}

const INPUT: &str = include_str!("../input/day16.txt");

problem!{
    tests => [
        run_part1 => {run(::std::io::Cursor::new(INPUT), 16, 1), "0c967262b13a4a86b9a7c71e2f2d4b799f6d3704322208bf9c22ed09d884455d"},
        run_part2 => {run(::std::io::Cursor::new(INPUT), 16, 1_000_000_000), "960dc4f93ced6a1e4be24bffde9b1e8dd2a7a6a4b1ffdadb3a01160bda705c14"},
    ];
}
