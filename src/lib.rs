#![feature(generators)]
#![feature(generator_trait)]
#![feature(conservative_impl_trait)]
#![feature(never_type)]
#![feature(inclusive_range_syntax)]
#![feature(iterator_step_by)]
#![feature(test)]

#[macro_use]
extern crate failure;
extern crate bit_vec;
extern crate structopt;
pub extern crate clap;
extern crate cgmath;
extern crate sha2;
extern crate test;

mod utils;
pub mod errors;
mod hex_slice;

/// Function used to hash answers.
///
/// Uses dynamic dispatch to avoid excessive monomorphization.
fn hash(value: &::std::fmt::Debug) -> String {
    use std::fmt::Write;
    use sha2::Digest;
    let mut hasher = sha2::Sha256::default();

    let mut buf = String::new();
    buf.write_fmt(format_args!("{:?}", value)).expect("a debug implementation failing");
    buf.shrink_to_fit();

    hasher.input(buf.as_bytes());

    let output = hasher.result();
    hex_slice::HexSlice::new(&output[..]).to_string()
}

#[macro_export]
macro_rules! problem {
    (tests => [$($test:tt)*]; $($rest:tt)*) => {
        mod benches {
            #[allow(unused)]
            use super::*;

            problem!(@bench $($test)*);
        }

        #[cfg(test)]
        mod problems {
            #[allow(unused)]
            use super::*;

            problem!(@test $($test)*);
        }

        #[allow(unused)]
        pub fn run_all(name: &str, spoil: bool) {
            problem!(@print spoil name $($test)*);
        }

        problem!($($rest)*);
    };

    (@bench $name:ident => ($test:expr, $exp:expr), $($rest:tt)*) => {
        #[bench]
        fn $name(b: &mut ::test::Bencher) {
            b.iter(|| $test);
        }

        problem!(@bench $($rest)*);
    };

    (@bench $name:ident => {$test:expr, $exp:expr}, $($rest:tt)*) => {
        #[bench]
        fn $name(b: &mut ::test::Bencher) {
            b.iter(|| $test);
        }

        problem!(@bench $($rest)*);
    };

    (@bench) => {};

    (@test $name:ident => ($test:expr, $exp:expr), $($rest:tt)*) => {
        #[test]
        fn $name() {
            assert_eq!($test, $exp);
        }

        problem!(@test $($rest)*);
    };

    (@test $name:ident => {$test:expr, $exp:expr}, $($rest:tt)*) => {
        #[test]
        fn $name() {
            assert_eq!($crate::hash(&$test).as_str(), $exp);
        }

        problem!(@test $($rest)*);
    };

    (@test) => {};

    (@print $spoil:ident $mod:ident $name:ident => ($test:expr, $exp:expr), $($rest:tt)*) => {
        println!("{:10} => {} = {:?}", stringify!($name), stringify!($test), $test);
        problem!(@print $spoil $mod $($rest)*)
    };

    (@print $spoil:ident $mod:ident $name:ident => {$test:expr, $exp:expr}, $($rest:tt)*) => {
        let start = ::std::time::Instant::now();

        let _r = $test;

        println!(
            "{:>6}::{:10} {} <{}>",
            $mod,
            stringify!($name),
            stringify!($test),
            $crate::hash(&_r)
        );

        if $spoil {
            println!(
                "{:>6}::{:10} = {:?}",
                $mod,
                stringify!($name),
                _r,
            );
        }

        let duration = ::std::time::Instant::now().duration_since(start);
        println!("{:>6}::{:10} {}s, {}ms", $mod, stringify!($name), duration.as_secs(), duration.subsec_nanos() / 1_000_000);

        problem!(@print $spoil $mod $($rest)*)
    };

    (@print $spoil:ident $mod:ident) => {};

    () => {
    };
}

macro_rules! modules {
    ($($mod:ident,)*) => {
        $(pub mod $mod;)*

        pub fn run_all(spoil: bool, filters: &[&str]) {
            println!("WARNING: SPOILERS ARE PRINTED!");
            $(
            if filters.iter().all(|f| stringify!($mod).contains(f)) {
                self::$mod::run_all(stringify!($mod), spoil);
            }
            )*
        }
    }
}

modules![
  day01,
  day02,
  day03,
  day04,
  day05,
  day06,
  day07,
  day08,
  day09,
  day10,
  day11,
  day12,
  day13,
  day14,
  day15,
  day16,
  day17,
  day18,
  day19,
  day20,
  day21,
  day22,
  day23,
  day24,
  day25,
];
