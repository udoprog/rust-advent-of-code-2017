#[macro_use]
extern crate advent_of_code;

use advent_of_code::day14;

entrypoint! {
    argument,
    "Day 14",
    input,
    {day14::part1(input)},
    {day14::part2(input)}
}
