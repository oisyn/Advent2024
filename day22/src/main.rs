#![allow(dead_code)]

use anyhow::Result;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day22")?;
    let mut total1 = 0;
    let mut total2 = 0;
    let mut map = vec![0; 1 << 20];
    let mut done = vec![false; 1 << 20];

    for mut n in input.lines().map(|l| l.parse::<u32>().unwrap()) {
        let mut seq = 0;
        let mut last = 0;
        for i in 0..2000 {
            n ^= (n << 6) & 0xffffff;
            n ^= n >> 5;
            n ^= (n << 11) & 0xffffff;

            let num = n % 10;
            let diff = num as i32 - last;
            last = num as i32;

            seq <<= 5;
            seq |= diff & 0x1f;
            seq &= 0xfffff;

            if i >= 4 && !done[seq as usize].exchange(true) {
                let bananas = &mut map[seq as usize];
                *bananas += num;
                total2 = total2.max(*bananas);
            }
        }
        done.fill(false);
        total1 += n as u64;
    }

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
