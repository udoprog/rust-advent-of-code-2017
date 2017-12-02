extern crate advent_of_code;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "day2", about = "Solve the Day 2 challenge")]
struct Opt {
    #[structopt(help = "input file")]
    input: String,

    #[structopt(short = "2", help = "Solve part 2")]
    part2: bool,
}

use advent_of_code::day2;

fn main() {
    let opt = Opt::from_args();

    if !opt.part2 {
        let result = day2::run(opt.input.as_str(), day2::minmax).unwrap();
        println!("result = {}", result);
    } else {
        let result = day2::run(opt.input.as_str(), day2::evendiv).unwrap();
        println!("result = {}", result);
    }
}
