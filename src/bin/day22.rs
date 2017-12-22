#[macro_use]
extern crate advent_of_code;
use advent_of_code::day22;

entrypoint! {
    file,
    "Day 22",
    input,
    {day22::part1(input, 10000)},
    {day22::part2(input, 10_000_000)}
}
