#[macro_use]
extern crate advent_of_code;

use advent_of_code::day07;

entrypoint!(
    file,
    "Day 7",
    input,
    {
        day07::run(input).map(|v| v.0)
    },
    {
        day07::run(input).map(|v| v.1)
    }
);
