use std::io::{Read, BufRead, BufReader};
use self::Action::*;

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
    Mul(u8, RegOrImm),
    Add(u8, RegOrImm),
    Mod(u8, RegOrImm),
    Jgz(RegOrImm, RegOrImm),
    Snd(RegOrImm),
    Rcv(u8),
}

#[derive(Debug)]
pub enum Action {
    Nothing,
    Halt,
    Store(i64),
}

pub trait Fragment {
    fn init(&mut self) -> Option<i64> {
        None
    }

    fn snd(&mut self, value: i64);

    fn rcv(&mut self, value: i64) -> Action;
}

pub struct Program<F> {
    registers: Registers,
    inst: Vec<Inst>,
    ip: usize,
    fragment: F,
}

impl<F> Program<F>
where
    F: Fragment,
{
    pub fn from_inst(inst: Vec<Inst>, fragment: F) -> Program<F> {
        let mut program = Program {
            registers: vec![0i64; 256],
            inst: inst,
            ip: 0,
            fragment: fragment,
        };

        if let Some(id) = program.fragment.init() {
            program.registers['p' as u8 as usize] = id;
        }

        program
    }

    pub fn run(&mut self) {
        use self::Inst::*;

        loop {
            let it = self.inst.get(self.ip).expect("ip overflow");

            match *it {
                Set(ref reg, ref arg) => {
                    self.registers[*reg as usize] = arg.value(&self.registers);
                }
                Mul(ref reg, ref arg) => {
                    self.registers[*reg as usize] *= arg.value(&self.registers);
                }
                Add(ref reg, ref arg) => {
                    self.registers[*reg as usize] += arg.value(&self.registers);
                }
                Mod(ref reg, ref arg) => {
                    self.registers[*reg as usize] %= arg.value(&self.registers);
                }
                Jgz(ref cond, ref offset) => {
                    let cond = cond.value(&self.registers);

                    if cond > 0 {
                        let o = offset.value(&self.registers);

                        if o < 0 {
                            self.ip = self.ip.checked_sub(-o as usize).expect("underflow");
                        } else {
                            self.ip = self.ip.checked_add(o as usize).expect("overflow");
                        }

                        continue;
                    }
                }
                Snd(ref arg) => {
                    let value = arg.value(&self.registers);
                    self.fragment.snd(value);
                }
                Rcv(ref reg) => {
                    let value = self.registers[*reg as usize];

                    match self.fragment.rcv(value) {
                        Halt => return,
                        Store(value) => self.registers[*reg as usize] = value,
                        Nothing => {}
                    }
                }
            }

            self.ip += 1;
        }
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
            "mul" => {
                let reg = reg(it.next().expect("no register"));
                let arg = parse_reg_or_imm(it.next().expect("no argument"));
                out.push(Inst::Mul(reg, arg));
            }
            "add" => {
                let reg = reg(it.next().expect("no register"));
                let arg = parse_reg_or_imm(it.next().expect("no argument"));
                out.push(Inst::Add(reg, arg));
            }
            "mod" => {
                let reg = reg(it.next().expect("no register"));
                let arg = parse_reg_or_imm(it.next().expect("no argument"));
                out.push(Inst::Mod(reg, arg));
            }
            "jgz" => {
                let cond = parse_reg_or_imm(it.next().expect("no register"));
                let arg = parse_reg_or_imm(it.next().expect("no argument"));
                out.push(Inst::Jgz(cond, arg));
            }
            "snd" => {
                let arg = parse_reg_or_imm(it.next().expect("no argument"));
                out.push(Inst::Snd(arg));
            }
            "rcv" => {
                let reg = reg(it.next().expect("no argument"));
                out.push(Inst::Rcv(reg));
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

pub fn part1<R: Read>(input: R) -> i64 {
    let inst = parse(input);

    let mut program = Program::from_inst(inst, Part1 { sent: 0 });
    program.run();

    return program.fragment.sent;

    pub struct Part1 {
        sent: i64,
    }

    impl Fragment for Part1 {
        fn snd(&mut self, value: i64) {
            self.sent = value;
        }

        fn rcv(&mut self, value: i64) -> Action {
            if value != 0 {
                if self.sent > 0 {
                    return Halt;
                }
            }

            Nothing
        }
    }
}

pub fn part2<R: Read>(input: R) -> u64 {
    use std::sync::mpsc::{Receiver, Sender, channel};

    let inst = parse(input);

    let (tx0, rx0) = channel();
    let (tx1, rx1) = channel();

    let mut p0 = Program::from_inst(inst.clone(), Part2::new(0, tx0, rx1));
    let mut p1 = Program::from_inst(inst.clone(), Part2::new(1, tx1, rx0));

    let mut attempts = 0;

    loop {
        p0.run();
        p1.run();

        if p0.fragment.send == p1.fragment.recv && p1.fragment.send == p0.fragment.recv {
            if attempts > 3 {
                return p1.fragment.send;
            }

            attempts += 1;
        } else {
            attempts = 0;
        }
    }

    #[derive(Debug)]
    pub struct Part2 {
        id: i64,
        send: u64,
        recv: u64,
        sender: Sender<i64>,
        receiver: Receiver<i64>,
    }

    impl Part2 {
        pub fn new(id: i64, sender: Sender<i64>, receiver: Receiver<i64>) -> Part2 {
            Part2 {
                id: id,
                send: 0,
                recv: 0,
                sender: sender,
                receiver: receiver,
            }
        }
    }

    impl Fragment for Part2 {
        fn init(&mut self) -> Option<i64> {
            Some(self.id)
        }

        fn snd(&mut self, value: i64) {
            self.send += 1;
            self.sender.send(value).expect("no receiver");
        }

        fn rcv(&mut self, _: i64) -> Action {
            use std::sync::mpsc::TryRecvError;

            match self.receiver.try_recv() {
                Ok(value) => {
                    self.recv += 1;
                    Store(value)
                }
                Err(TryRecvError::Empty) => Halt,
                Err(e) => panic!("unexpected error: {}", e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static INPUT: &str = include_str!("../input/day18.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(Cursor::new(INPUT)), 3423);
    }

    #[test]
    fn test_example1() {
        assert_eq!(
            part2(Cursor::new(
                "snd 1\nsnd 2\nsnd p\nrcv a\nrcv b\nrcv c\nrcv d",
            )),
            3
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(Cursor::new(INPUT)), 7493);
    }
}
