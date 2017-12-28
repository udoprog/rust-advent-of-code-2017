use self::State::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

/// NB: this is a hand-translated version of the input.
pub fn run() -> u64 {
    let mut state = A;
    let mut pos = 0i64;
    let mut values = HashSet::new();
    let checksum_step = 12_368_930;

    for _ in 0..checksum_step {
        let value = values.contains(&pos);

        match (state, value) {
            (A, false) => {
                values.insert(pos);
                pos += 1;
                state = B;
            }
            (A, true) => {
                values.remove(&pos);
                pos += 1;
                state = C;
            }
            (B, false) => {
                values.remove(&pos);
                pos -= 1;
                state = A;
            }
            (B, true) => {
                values.remove(&pos);
                pos += 1;
                state = D;
            }
            (C, false) => {
                values.insert(pos);
                pos += 1;
                state = D;
            }
            (C, true) => {
                values.insert(pos);
                pos += 1;
                state = A;
            }
            (D, false) => {
                values.insert(pos);
                pos -= 1;
                state = E;
            }
            (D, true) => {
                values.remove(&pos);
                pos -= 1;
                state = D;
            }
            (E, false) => {
                values.insert(pos);
                pos += 1;
                state = F;
            }
            (E, true) => {
                values.insert(pos);
                pos -= 1;
                state = B;
            }
            (F, false) => {
                values.insert(pos);
                pos += 1;
                state = A;
            }
            (F, true) => {
                values.insert(pos);
                pos += 1;
                state = E;
            }
        }
    }

    values.len() as u64
}

problem!{
    tests => [
        both => {run(), "fa014137b3ea9af6a90c0a86a1d099e46f7e56d6eb33db1ad1ec4bdac68c3caa"},
    ];
}
