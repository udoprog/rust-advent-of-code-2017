#[macro_use]
extern crate advent_of_code;
use advent_of_code::day23;

entrypoint! {
    file,
    "Day 23",
    input,
    {day23::part1(input)},
    {day23::part2()}
}
