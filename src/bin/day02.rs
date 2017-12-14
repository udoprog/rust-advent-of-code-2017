#[macro_use]
extern crate advent_of_code;

use advent_of_code::day02;

entrypoint!(
    file,
    "Day 2",
    input,
    {
        day02::run(input, day02::minmax)
    },
    {
        day02::run(input, day02::evendiv)
    }
);
