#[macro_export]
macro_rules! entrypoint {
    (file, $name:expr, $input:ident, $part1:block, $part2:block) => {
        entrypoint!(
            @internal
            $name,
            $input,
            $part1,
            $part2,
            { ::std::fs::File::open($input).expect("input file not readable") }
        );
    };

    (argument, $name:expr, $input:ident, $part1:block, $part2:block) => {
        entrypoint!(
            @internal
            $name,
            $input,
            $part1,
            $part2,
            { $input }
        );
    };

    (@internal $name:expr, $input:ident, $part1:block, $part2:block, $filter:block) => {
        fn main() {
            let matches = $crate::clap::App::new($name)
                .arg($crate::clap::Arg::with_name("input").required(true))
                .arg($crate::clap::Arg::with_name("2").short("2").help("Solve Part 2"))
                .get_matches();

            let $input = matches.value_of("input").expect("no input");
            let $input = $filter;

            if !matches.is_present("2") {
                let result = $part1;
                println!("result = {}", result.unwrap());
            } else {
                let result = $part2;
                println!("result = {}", result.unwrap());
            }
        }
    };
}
