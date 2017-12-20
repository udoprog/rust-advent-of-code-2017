#[macro_use]
extern crate advent_of_code;
use advent_of_code::day20;

entrypoint! {
    file,
    "Day 20",
    input,
    {day20::part1(input)},
    {day20::part2(input)}
}
