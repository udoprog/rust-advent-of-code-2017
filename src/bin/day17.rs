#[macro_use]
extern crate advent_of_code;
use advent_of_code::day17;

entrypoint! {
    argument,
    "Day 17",
    input,
    {day17::part1(input.parse().expect("bad number"), 2017)},
    {day17::part2(input.parse().expect("bad number"), 50_000_000).expect("no result")}
}
