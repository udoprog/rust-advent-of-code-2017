extern crate advent_of_code;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "day1", about = "Solve the Day 1 challenge")]
struct Opt {
    #[structopt(help = "input file")]
    input: String,

    #[structopt(short = "2", help = "Solve part 2")]
    part2: bool,
}

fn main() {
    let opt = Opt::from_args();

    if !opt.part2 {
        let result = advent_of_code::day1::run(opt.input.as_str(), |_| 1).unwrap();
        println!("result = {}", result);
    } else {
        let result = advent_of_code::day1::run(opt.input.as_str(), |size| size / 2).unwrap();
        println!("result = {}", result);
    }
}
