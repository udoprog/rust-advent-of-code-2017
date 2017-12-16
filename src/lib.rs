#![feature(generators)]
#![feature(generator_trait)]
#![feature(conservative_impl_trait)]
#![feature(never_type)]
#![feature(inclusive_range_syntax)]
#![feature(iterator_step_by)]

#[macro_use]
extern crate failure;
extern crate bit_vec;
extern crate structopt;
pub extern crate clap;

#[macro_use]
mod macros;
mod utils;
pub mod errors;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
