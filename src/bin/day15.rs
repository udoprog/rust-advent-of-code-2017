#[macro_use]
extern crate advent_of_code;
use advent_of_code::day15;

entrypoint!(
    file,
    "Day 15",
    input,
    {
        day15::part1(input)
    },
    {
        day15::part2(input)
    }
);
