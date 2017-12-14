#[macro_use]
extern crate advent_of_code;

use advent_of_code::day08;

entrypoint! {
    file,
    "Day 8",
    input,
    {day08::run(input).map(|v| v.0)},
    {day08::run(input).map(|v| v.1)}
}
