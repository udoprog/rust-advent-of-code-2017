#[macro_use]
extern crate advent_of_code;
use advent_of_code::day12;

entrypoint!(
    file,
    "Day 12",
    input,
    {
        day12::run(input).map(|v| v.0)
    },
    {
        day12::run(input).map(|v| v.1)
    }
);
