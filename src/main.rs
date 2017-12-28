extern crate advent_of_code;

use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    let mut spoil = false;
    let mut filters = Vec::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--spoil" => spoil = true,
            arg => filters.push(arg.to_string()),
        }
    }

    let filters: Vec<&str> = filters.iter().map(|s| s.as_str()).collect();
    advent_of_code::run_all(spoil, &filters);
}
