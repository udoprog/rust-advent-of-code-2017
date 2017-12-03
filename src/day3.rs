use failure::Error;

use std::ops::{Generator, GeneratorState};
use std::collections::HashMap;
use self::GeneratorState::*;

/// Walk the memory layout.
///
/// Note 1: Returns the `never` type, which permits Rust to realize that this function never
/// returns. This also means that attempting to pattern-match with anything but `Yielded` (see
/// below) will cause a compilation error. This behavior is because `Returned` (the other
/// `GeneratorState` variant) can never be inhabited.
pub fn memory_walker() -> impl Generator<Yield = (i64, i64), Return = !> {
    || {
        yield (0, 0);
        yield (1, 0);

        let mut x = 1i64;
        let mut y = 0i64;
        let mut size = 0u64;

        loop {
            // max length of a side
            let side = 3 + (2 * size);

            // right
            for _ in 2..side {
                y += 1;
                yield (x, y);
            }

            // top
            for _ in 1..side {
                x -= 1;
                yield (x, y);
            }

            // left
            for _ in 1..side {
                y -= 1;
                yield (x, y);
            }

            // bottom
            for _ in 0..side {
                x += 1;
                yield (x, y);
            }

            size += 1;
        }
    }
}

pub fn run_with_larger(target: u64) -> Result<u64, Error> {
    let mut storage: HashMap<(i64, i64), u64> = HashMap::new();

    let mut index = 1;
    let mut walk = memory_walker();

    loop {
        // Note: this is safe since the generator never returns. See above.
        let Yielded((x, y)) = walk.resume();

        let value = match index {
            1 => {
                storage.insert((x, y), 1);
                1
            },
            _ => {
                let value = cell_value(&storage, x, y);
                storage.insert((x, y), value);
                value
            }
        };

        if value > target {
            return Ok(value);
        }

        index += 1;
    }

    fn cell_value(storage: &HashMap<(i64, i64), u64>, x: i64, y: i64) -> u64 {
        // Note 1: HashMap::get returns an Option, which implements IntoIterator. This allows it to
        // be used in flat_map.
        // Note 2: Derefing (`*v`) a reference to a primitive numeric type will copy it.
        [
            &(x - 1, y - 1), &(x  , y - 1), &(x + 1, y - 1),
            &(x - 1, y    ), /* (x, y)   */ &(x + 1, y    ),
            &(x - 1, y + 1), &(x  , y + 1), &(x + 1, y + 1),
        ].into_iter().flat_map(|k| storage.get(k)).map(|v| *v).sum()
    }
}

pub fn run(pos: u64) -> Result<i64, Error> {
    let mut index = 1;
    let mut walk = memory_walker();

    loop {
        // Note: this is safe since the generator never returns. See above.
        let Yielded((x, y)) = walk.resume();

        if index == pos {
            return Ok(i64::abs(x) + i64::abs(y));
        }

        index += 1;
    }
}
