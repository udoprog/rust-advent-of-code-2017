#[macro_use]
extern crate advent_of_code;
use advent_of_code::day13;

entrypoint!(
    file,
    "Day 13",
    input,
    {
        day13::part1(input)
    },
    {
        day13::part2(input)
    }
);
