#[macro_use]
extern crate advent_of_code;
use advent_of_code::day10;

entrypoint! {
    file,
    "Day 10",
    input,
    {day10::part1(input, 255)},
    {day10::part2(input)}
}
