use std::io::{BufRead, BufReader, Read};
use failure::Error;
use std::collections::HashMap;

use self::Op::*;
use self::Cond::*;

type Registers = HashMap<String, i64>;

enum Op {
    Inc(String, i64),
    Dec(String, i64),
}

impl Op {
    /// Apply the given operation.
    pub fn apply(&self, registers: &mut Registers, highest: &mut i64) {
        let reg = match *self {
            Inc(ref reg, _) | Dec(ref reg, _) => reg,
        };

        let value = registers.entry(reg.to_string()).or_insert_with(
            Default::default,
        );

        match *self {
            Inc(_, number) => *value += number,
            Dec(_, number) => *value -= number,
        };

        *highest = i64::max(*highest, *value);
    }
}

enum Cond {
    Gt(String, i64),
    GtEq(String, i64),
    Lt(String, i64),
    LtEq(String, i64),
    Eq(String, i64),
    NotEq(String, i64),
}

impl Cond {
    /// Test if the given condition applies.
    pub fn test(&self, registers: &mut Registers) -> bool {
        let reg = match *self {
            Gt(ref reg, _) |
            GtEq(ref reg, _) |
            Lt(ref reg, _) |
            LtEq(ref reg, _) |
            Eq(ref reg, _) |
            NotEq(ref reg, _) => reg,
        };

        let value = registers.entry(reg.to_string()).or_insert_with(
            Default::default,
        );

        match *self {
            Gt(_, number) => *value > number,
            GtEq(_, number) => *value >= number,
            Lt(_, number) => *value < number,
            LtEq(_, number) => *value <= number,
            Eq(_, number) => *value == number,
            NotEq(_, number) => *value != number,
        }
    }
}

fn parse(input: &str) -> (Op, Cond) {
    let mut it = input.trim().split(' ');
    let target = it.next().expect("target").to_string();

    let op = it.next().expect("op");
    let number = it.next().expect("number").parse::<i64>().expect(
        "valid number",
    );

    let op = match op {
        "inc" => Inc(target, number),
        "dec" => Dec(target, number),
        op => panic!("llegal op: {}", op),
    };

    it.next().expect("if-separator");

    let cond_reg = it.next().expect("cond-reg").to_string();
    let cond = it.next().expect("cond");
    let cond_number = it.next().expect("cond-number").parse::<i64>().expect(
        "valid cond-number",
    );

    let cond = match cond {
        "<" => Lt(cond_reg, cond_number),
        "<=" => LtEq(cond_reg, cond_number),
        ">" => Gt(cond_reg, cond_number),
        ">=" => GtEq(cond_reg, cond_number),
        "==" => Eq(cond_reg, cond_number),
        "!=" => NotEq(cond_reg, cond_number),
        _ => panic!("illegal cond: {}", cond),
    };

    (op, cond)
}

pub fn run<R: Read>(reader: R) -> Result<(i64, i64), Error> {
    let mut data = String::new();
    let mut reader = BufReader::new(reader);
    let mut registers = Registers::new();
    let mut highest = 0i64;

    while reader.read_line(&mut data)? > 0 {
        {
            let (op, cond) = parse(data.as_str());

            if cond.test(&mut registers) {
                op.apply(&mut registers, &mut highest);
            }
        }

        data.clear();
    }

    Ok((
        registers.values().max().map(|v| *v).unwrap_or_else(
            Default::default,
        ),
        highest,
    ))
}

const INPUT: &str = include_str!("../input/day8.txt");

problem!{
    tests => [
        both => {run(::std::io::Cursor::new(INPUT)), "4e5de0c145806d4999a9aed7bf75b38b53dcb170dd75073bfe848e08be48dc59"},
    ];
}
