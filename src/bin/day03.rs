#[macro_use]
extern crate advent_of_code;

use advent_of_code::day03;

entrypoint!(
    argument,
    "Day 3",
    input,
    {
        day03::run(input.parse().expect("bad input"))
    },
    {
        day03::run_with_larger(input.parse().expect("bad input"))
    }
);
