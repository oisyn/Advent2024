#![allow(dead_code)]

use util::*;

struct VM {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    pc: usize,
    opcodes: Vec<u8>,
}

impl VM {
    fn run(&mut self, mut out: impl FnMut(u8) -> bool) -> bool {
        loop {
            if self.pc >= self.opcodes.len() {
                return false;
            }
            let opcode = self.opcodes[self.pc.post_inc()];
            let operand = self.opcodes[self.pc.post_inc()];
            let combo = match operand {
                x @ 0..=3 => x as u64,
                4 => self.reg_a,
                5 => self.reg_b,
                6 => self.reg_c,
                7 => u64::MAX,
                _ => unreachable!("Invalid operand: {}", operand),
            };
            match opcode {
                0 => self.reg_a >>= combo,
                1 => self.reg_b ^= operand as u64,
                2 => self.reg_b = combo as u64 & 7,
                3 => {
                    if self.reg_a != 0 {
                        self.pc = (operand * 2) as usize;
                    }
                }
                4 => self.reg_b ^= self.reg_c,
                5 => {
                    if out(combo as u8 & 7) {
                        return true;
                    }
                }
                6 => self.reg_b = self.reg_a >> combo,
                7 => self.reg_c = self.reg_a >> combo,
                _ => unreachable!("Invalid opcode: {}", opcode),
            };
        }
    }
}

#[aoc_day]
fn solve(input: Input) -> impl AocResult {
    let mut lines = input.lines();
    let reg_a = Parser::new(lines.next().unwrap())
        .expect("Register A: ")
        .parse()
        .unwrap();
    let reg_b = Parser::new(lines.next().unwrap())
        .expect("Register B: ")
        .parse()
        .unwrap();
    let reg_c = Parser::new(lines.next().unwrap())
        .expect("Register C: ")
        .parse()
        .unwrap();
    lines.next();
    let opcodes = Parser::new(lines.next().unwrap())
        .expect("Program: ")
        .parse_iter::<u8>(",")
        .collect::<Vec<_>>();

    let mut vm = VM {
        reg_a,
        reg_b,
        reg_c,
        pc: 0,
        opcodes,
    };

    let mut out = String::with_capacity(32);
    vm.run(|c| {
        if !out.is_empty() {
            out.push(',');
        }
        out.push((c + b'0') as char);
        false
    });

    let mut part2 = u64::MAX;
    let orig_opcodes = vm.opcodes.clone();
    let mut upper_bits = Vec::with_capacity(32);
    let mut next_upper_bits = Vec::with_capacity(32);
    let mut set = vec![false; 1024];
    upper_bits.push(0);

    'outer: for num in 1..=orig_opcodes.len() {
        let wanted = &orig_opcodes[orig_opcodes.len() - num..];

        // println!("{wanted:?} {} combos", upper_bits.len());
        // for &l in upper_bits.iter() {
        //     println!("{l:064b}");
        // }

        for &upper in &upper_bits {
            for lower in 0..8_u64 {
                let reg_a = upper | lower;
                if set[(reg_a & 1023) as usize] {
                    continue;
                }
                vm.reg_a = reg_a;
                vm.reg_b = reg_b;
                vm.reg_c = reg_c;
                vm.pc = 0;

                let mut pos = 0;
                let halted = !vm.run(|c| pos >= wanted.len() || c != wanted[pos.post_inc()]);

                if halted && pos >= wanted.len() {
                    if pos == orig_opcodes.len() {
                        part2 = reg_a;
                        break 'outer;
                    } else {
                        set[(reg_a & 1023) as usize] = true;
                        next_upper_bits.push(reg_a << 3);
                    }
                }
            }
        }
        std::mem::swap(&mut upper_bits, &mut next_upper_bits);
        set.fill(false);
        next_upper_bits.clear();
    }

    (out, part2)
}
