extern crate advent_of_code;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "day3", about = "Solve the Day 3 challenge")]
struct Opt {
    #[structopt(help = "input number")]
    input: u32,

    #[structopt(short = "2", help = "Solve part 2")]
    part2: bool,
}

use advent_of_code::day3;

fn main() {
    let opt = Opt::from_args();

    if !opt.part2 {
        let result = day3::run(opt.input as u64).unwrap();
        println!("result = {}", result);
    } else {
        let result = day3::run_with_larger(opt.input as u64).unwrap();
        println!("result = {}", result);
    }
}
