#[macro_use]
extern crate advent_of_code;

use advent_of_code::day05;

entrypoint!(
    file,
    "Day 5",
    input,
    {
        day05::run(input, |v| v + 1)
    },
    {
        day05::run(input, |v| if v < 3 { v + 1 } else { v - 1 })
    }
);
