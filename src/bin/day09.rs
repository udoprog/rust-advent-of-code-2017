#[macro_use]
extern crate advent_of_code;
use advent_of_code::day09;

entrypoint! {
    file,
    "Day 9",
    input,
    {day09::run(input).map(|v| v.0)},
    {day09::run(input).map(|v| v.1)}
}
