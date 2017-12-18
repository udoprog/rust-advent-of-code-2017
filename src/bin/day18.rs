#[macro_use]
extern crate advent_of_code;
use advent_of_code::day18;

entrypoint! {
    file,
    "Day 18",
    input,
    {Ok(day18::part1(input)) as Result<_, &str>},
    {Ok(day18::part2(input)) as Result<_, &str>}
}
