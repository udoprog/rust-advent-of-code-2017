#[macro_use]
extern crate advent_of_code;

use advent_of_code::day06;

entrypoint!(
    file,
    "Day 6",
    input,
    {
        day06::run(input, day06::index)
    },
    {
        day06::run(input, day06::distance)
    }
);
