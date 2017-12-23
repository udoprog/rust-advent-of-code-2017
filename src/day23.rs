use std::io::{Read, BufRead, BufReader};

type Registers = Vec<i64>;

#[derive(Debug, Clone)]
pub enum RegOrImm {
    Reg(u8),
    Immediate(i64),
}

impl RegOrImm {
    pub fn value(&self, registers: &Registers) -> i64 {
        use self::RegOrImm::*;

        match *self {
            Reg(reg) => registers[reg as usize],
            Immediate(value) => value,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Inst {
    Set(u8, RegOrImm),
    Sub(u8, RegOrImm),
    Mul(u8, RegOrImm),
    Jnz(RegOrImm, RegOrImm),
}

pub struct Program {
    registers: Registers,
    inst: Vec<Inst>,
    ip: usize,
}

impl Program {
    pub fn from_inst(inst: Vec<Inst>) -> Program {
        Program {
            registers: vec![0i64; 256],
            inst: inst,
            ip: 0,
        }
    }

    pub fn register(&self, reg: char) -> i64 {
        self.registers[reg as usize]
    }

    pub fn run(&mut self) -> u64 {
        use self::Inst::*;

        let mut mul = 0;

        loop {
            let it = match self.inst.get(self.ip) {
                Some(ip) => ip,
                None => break,
            };

            match *it {
                Set(ref reg, ref arg) => {
                    self.registers[*reg as usize] = arg.value(&self.registers);
                }
                Sub(ref reg, ref arg) => {
                    self.registers[*reg as usize] -= arg.value(&self.registers);
                }
                Mul(ref reg, ref arg) => {
                    mul += 1;
                    self.registers[*reg as usize] *= arg.value(&self.registers);
                }
                Jnz(ref cond, ref offset) => {
                    let cond = cond.value(&self.registers);

                    if cond != 0 {
                        let o = offset.value(&self.registers);

                        if o < 0 {
                            self.ip = self.ip.checked_sub(-o as usize).expect("underflow");
                        } else {
                            self.ip = self.ip.checked_add(o as usize).expect("overflow");
                        }

                        continue;
                    }
                }
            }

            self.ip += 1;
        }

        mul
    }
}

fn parse<R: Read>(input: R) -> Vec<Inst> {
    let mut out = Vec::new();

    for line in BufReader::new(input).lines() {
        let line = line.expect("bad line");

        let mut it = line.split_whitespace();

        match it.next().expect("no instruction") {
            "set" => {
                let reg = reg(it.next().expect("no register"));
                let arg = parse_reg_or_imm(it.next().expect("no argument"));
                out.push(Inst::Set(reg, arg));
            }
            "sub" => {
                let reg = reg(it.next().expect("no register"));
                let arg = parse_reg_or_imm(it.next().expect("no argument"));
                out.push(Inst::Sub(reg, arg));
            }
            "mul" => {
                let reg = reg(it.next().expect("no register"));
                let arg = parse_reg_or_imm(it.next().expect("no argument"));
                out.push(Inst::Mul(reg, arg));
            }
            "jnz" => {
                let cond = parse_reg_or_imm(it.next().expect("no register"));
                let arg = parse_reg_or_imm(it.next().expect("no argument"));
                out.push(Inst::Jnz(cond, arg));
            }
            inst => panic!("unknown instruction: {}", inst),
        }
    }

    return out;

    fn reg(input: &str) -> u8 {
        input.chars().next().expect("empty string") as u8
    }

    fn parse_reg_or_imm(input: &str) -> RegOrImm {
        if let Ok(v) = input.parse::<i64>() {
            return RegOrImm::Immediate(v);
        }

        let c = input.chars().next().expect("empty string");
        RegOrImm::Reg(c as u8)
    }
}

pub fn part1<R: Read>(input: R) -> u64 {
    let inst = parse(input);

    let mut program = Program::from_inst(inst);
    program.run()
}

pub fn part2() -> i64 {
    // NB: this is a hand-disassembled version of my input.
    let mut b: i64 = 57;
    let mut c = b;
    let mut h: i64 = 0;

    if true {
        b = b * 100 + 100000;
        c = b + 17000;
    }

    loop {
        let upper = (b as f64).sqrt() as i64 + 1;

        for d in 2..upper {
            if b % d == 0 {
                h = h + 1;
                break;
            }
        }

        if b == c {
            break;
        }

        b += 17;
    }

    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static INPUT: &str = include_str!("../input/day23.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(Cursor::new(INPUT)), 3025);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 915);
    }
}
