use std::io::{Read, BufRead, BufReader};
use std::collections::VecDeque;

pub fn parse<R: Read>(input: R) -> Vec<(u32, u32)> {
    let mut parts = Vec::new();

    for line in BufReader::new(input).lines() {
        let line = line.expect("bad line");
        let mut it = line.trim().split('/');
        let left: u32 = it.next().expect("no lhs").parse().expect("bad number");
        let right: u32 = it.next().expect("no rhs").parse().expect("bad number");

        parts.push((left, right));
    }

    parts
}

pub fn run<R: Read>(input: R) -> (u32, u32) {
    let parts = parse(input);

    let mut queue = VecDeque::new();

    // initialize
    for (init, _) in parts.iter().enumerate().filter(|&(_, ref v)| v.0 == 0 || v.1 == 0) {
        let mut parts = parts.clone();
        let v = parts.remove(init);
        queue.push_back((other(v, 0), vec![v], parts));
    }

    let mut best = 0;
    let mut longest = (0, 0);

    while let Some((slot, path, parts)) = queue.pop_front() {
        if parts.len() == 0 {
            continue;
        }

        let mut any = false;

        for (init, _) in parts.iter().enumerate().filter(|&(_, ref v)| v.0 == slot || v.1 == slot) {
            any = true;

            let mut parts = parts.clone();
            let v = parts.remove(init);

            let mut path = path.clone();
            path.push(v);

            queue.push_back((other(v, slot), path, parts));
        }

        if !any {
            let s = sum(&path);

            best = u32::max(s, best);

            if path.len() >= longest.0 {
                if path.len() > longest.0 {
                    longest = (path.len(), s);
                } else {
                    longest = (path.len(), u32::max(longest.1, s));
                }
            }
        }
    }

    return (best, longest.1);

    fn other(v: (u32, u32), current: u32) -> u32 {
        if v.0 == current {
            v.1
        } else {
            v.0
        }
    }

    fn sum(v: &[(u32, u32)]) -> u32 {
        v.iter().map(|v| v.0 + v.1).sum()
    }
}

const INPUT: &str = include_str!("../input/day24.txt");

problem!{
    tests => [
        both => {run(::std::io::Cursor::new(INPUT)), "b6a10f78fcdb219575ef11cc79b5a9fe0dbfafcc1ae09f6b560fa5104d23e7c4"},
    ];
}
