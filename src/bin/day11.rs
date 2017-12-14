#[macro_use]
extern crate advent_of_code;
use advent_of_code::day11;

entrypoint! {
    file,
    "Day 11",
    input,
    {day11::run(input).map(|v| v.0)},
    {day11::run(input).map(|v| v.1)}
}
