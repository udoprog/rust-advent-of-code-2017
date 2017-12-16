#[macro_use]
extern crate advent_of_code;
use advent_of_code::day16;

entrypoint!(
    file,
    "Day 16",
    input,
    {
        day16::run(input, 16, 1)
    },
    {
        day16::run(input, 16, 1_000_000_000)
    }
);
