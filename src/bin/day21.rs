#[macro_use]
extern crate advent_of_code;
use advent_of_code::day21;

entrypoint! {
    file,
    "Day 21",
    input,
    {day21::run(input, 5)},
    {day21::run(input, 18)}
}
