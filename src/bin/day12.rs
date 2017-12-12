extern crate advent_of_code;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::fs::File;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "day12", about = "Solve the Day 12 challenge")]
struct Opt {
    #[structopt(help = "input file")]
    input: String,

    #[structopt(short = "2", help = "Solve part 2")]
    part2: bool,
}

use advent_of_code::day12;

fn main() {
    let opt = Opt::from_args();

    let mut input = File::open(opt.input.as_str()).expect("input file to be readable");

    if !opt.part2 {
        let result = day12::run(&mut input).unwrap().0;
        println!("result = {}", result);
    } else {
        let result = day12::run(&mut input).unwrap().1;
        println!("result = {}", result);
    }
}
