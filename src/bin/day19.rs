#[macro_use]
extern crate advent_of_code;
use advent_of_code::day19;

entrypoint! {
    file,
    "Day 19",
    input,
    {day19::run(input).0},
    {day19::run(input).1}
}
