#[macro_use]
extern crate advent_of_code;

use advent_of_code::day01;

entrypoint! {
    file,
    "Day 1",
    input,
    {day01::run(input, |_| 1)},
    {day01::run(input, |size| size / 2)}
}
