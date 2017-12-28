use std::io::Read;
use failure::Error;
use std::str;

fn score(input: &str) -> (u64, u64) {
    let mut garbage = 0u64;

    let mut total = 0u64;
    let mut depth = 0u64;

    let mut input = input.chars();

    while let Some(c) = input.next() {
        match c {
            '!' => {
                input.next();
            }
            '{' => {
                depth += 1;
            }
            '}' => {
                total += depth;
                depth -= 1;
            }
            '<' => {
                while let Some(c) = input.next() {
                    match c {
                        '!' => {
                            input.next();
                        }
                        '>' => break,
                        _ => garbage += 1,
                    }
                }
            }
            _ => {}
        }
    }

    (total, garbage)
}

pub fn run<R: Read>(mut reader: R) -> Result<(u64, u64), Error> {
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    let score = score(data.as_str());
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_one() {
        assert_eq!((1, 0), run(Cursor::new("{}")).unwrap());
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            (9, 8),
            run(Cursor::new("{{<ab>},{<ab>},{<ab>},{<ab>}}")).unwrap()
        );
        assert_eq!(
            (9, 0),
            run(Cursor::new("{{<!!>},{<!!>},{<!!>},{<!!>}}")).unwrap()
        );
        assert_eq!((16, 0), run(Cursor::new("{{{},{},{{}}}}")).unwrap());
    }
}

const INPUT: &str = include_str!("../input/day9.txt");

problem!{
    tests => [
        both => {run(::std::io::Cursor::new(INPUT)), "70e47166875e064f06a9d99abae87064c742b69039b7c9d410b0fbda7a6ee579"},
    ];
}
