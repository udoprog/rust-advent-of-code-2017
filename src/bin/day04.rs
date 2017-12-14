#[macro_use]
extern crate advent_of_code;

use advent_of_code::day04;

entrypoint!(
    file,
    "Day 4",
    input,
    {
        day04::run(input, day04::unique)
    },
    {
        day04::run(input, day04::unique_anagrams)
    }
);
