#[macro_use]
extern crate advent_of_code;
use advent_of_code::day24;

entrypoint! {
    file,
    "Day 24",
    input,
    {day24::run(input).0},
    {day24::run(input).1}
}
