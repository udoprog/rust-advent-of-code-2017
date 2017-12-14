extern crate advent_of_code;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "day14", about = "Solve the Day 14 challenge")]
struct Opt {
    #[structopt(help = "input data")]
    input: String,

    #[structopt(short = "2", help = "Solve part 2")]
    part2: bool,
}

use advent_of_code::day14;

fn main() {
    let opt = Opt::from_args();

    if !opt.part2 {
        let result = day14::part1(opt.input.as_str()).unwrap();
        println!("result = {}", result);
    } else {
        let result = day14::part2(opt.input.as_str()).unwrap();
        println!("result = {}", result);
    }
}
