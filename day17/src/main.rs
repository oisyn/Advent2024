#![allow(dead_code)]

use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;
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
            // println!(
            //     "{opcode} {operand}/{combo} | A:{reg_a} B:{reg_b} C:{reg_c} PC:{pc} OUT:{out}"
            // );
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

fn main() -> Result<()> {
    let input = open_input("day17")?;
    let mut lines = input.lines();
    let reg_a = Parser::new(lines.next().unwrap())
        .expect("Register A: ")
        .parse()
        .unwrap();
    let reg_b = Parser::new(lines.next().unwrap())
        .expect("Register A: ")
        .parse()
        .unwrap();
    let reg_c = Parser::new(lines.next().unwrap())
        .expect("Register A: ")
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

    let mut out = String::new();
    vm.run(|c| {
        if !out.is_empty() {
            out.push(',');
        }
        out.push((c + b'0') as char);
        false
    });

    let mut part2 = u64::MAX;
    let orig_opcodes = vm.opcodes.clone();
    let mut lower_bits = HashSet::with_capacity(10000);
    lower_bits.insert(0);
    let mut next_lower_bits = HashSet::with_capacity(10000);
    for num in 1..=orig_opcodes.len() {
        let wanted = &orig_opcodes[0..num];
        // println!("{wanted:?} {} combos", lower_bits.len());

        let shift = (num - 1) as u64 * 3;
        let mask = (1 << shift + 3) - 1;

        // println!("MASK****{:056b}", 1023_u64 << shift);
        // for &l in lower_bits.iter().sorted() {
        //     println!("{l:064b}");
        // }

        for i in 0..1024_u64 {
            let upper = i << shift;
            for &lower in &lower_bits {
                let reg_a = upper | lower;
                vm.reg_a = reg_a;
                vm.reg_b = reg_b;
                vm.reg_c = reg_c;
                vm.pc = 0;

                // print!("{reg_a}: ");
                let mut pos = 0;
                let halted = !vm.run(|c| {
                    // print!("{c},");
                    pos >= orig_opcodes.len() || c != orig_opcodes[pos.post_inc()]
                });
                if pos >= wanted.len() {
                    if halted && (pos == orig_opcodes.len()) {
                        if part2 > reg_a {
                            part2 = reg_a;
                        }
                        next_lower_bits.insert(reg_a);
                    } else if !halted && pos != orig_opcodes.len() && pos >= wanted.len() {
                        // println!("found {reg_a} for {wanted:?}");
                        next_lower_bits.insert(reg_a & mask);
                    }
                }
                // println!("");
            }
        }
        std::mem::swap(&mut lower_bits, &mut next_lower_bits);
        next_lower_bits.clear();

        if part2 < u64::MAX {
            break;
        }
    }

    drop(input);
    println!("{out} - {part2}");

    // for &l in lower_bits.iter().sorted() {
    //     println!("{l:064b}");
    // }

    Ok(())
}
