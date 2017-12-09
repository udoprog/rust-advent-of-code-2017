use std::io::{Read};
use failure::Error;
use std::str;

struct Lexer<'a> {
    chars: str::Chars<'a>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.chars.next() {
            if c == '!' {
                self.chars.next();
                continue;
            }

            return Some(c);
        }

        None
    }
}

fn score(input: Lexer) -> (u64, u64) {
    let mut garbage = 0u64;

    let mut total = 0u64;
    let mut depth = 0u64;

    let mut input = input.peekable();

    while let Some(c) = input.next() {
        if c == '{' {
            depth += 1;
            continue;
        }

        if c == '}' && depth > 0 {
            total += depth;
            depth -= 1;
            continue;
        }

        if c == '<' {
            while let Some(&'<') = input.peek() {
                garbage += 1;
                input.next();
            }

            while let Some(&c) = input.peek() {
                if c == '>' {
                    break;
                }

                garbage += 1;
                input.next();
            }

            continue;
        }
    }

    (total, garbage)
}

pub fn run<R: Read>(mut reader: R) -> Result<(u64, u64), Error> {
    let mut data = String::new();
    reader.read_to_string(&mut data)?;

    let lexer = Lexer {
        chars: data.chars(),
    };

    let score = score(lexer);
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static INPUT: &str = include_str!("../input/day9.txt");

    #[test]
    fn test_all() {
        assert_eq!((14204, 6622), run(Cursor::new(INPUT)).unwrap());
    }

    #[test]
    fn test_one() {
        assert_eq!((1, 0), run(Cursor::new("{}")).unwrap());
    }

    #[test]
    fn test_example2() {
        assert_eq!((9, 8), run(Cursor::new("{{<ab>},{<ab>},{<ab>},{<ab>}}")).unwrap());
        assert_eq!((9, 0), run(Cursor::new("{{<!!>},{<!!>},{<!!>},{<!!>}}")).unwrap());
        assert_eq!((16, 0), run(Cursor::new("{{{},{},{{}}}}")).unwrap());
    }
}
