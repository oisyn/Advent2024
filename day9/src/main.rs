#![allow(non_camel_case_types)]

use anyhow::Result;
use std::ops::*;
use util::*;

#[cfg(feature = "u128")]
type uresult = u128;

#[cfg(not(feature = "u128"))]
type uresult = u64;

fn sum(from: u32, to: u32) -> uresult {
    ((to as u64 * to.add(1) as u64 - from as u64 * from.sub(1) as u64) / 2) as uresult
}

const INVALID: u32 = u32::MAX;

struct Gap {
    offset: u32,
    len: u32,
    links: [(u32, u32); 9],
}
struct GapList {
    gaps: Vec<Gap>,
    first: [u32; 9],
    last: [u32; 9],
}

impl GapList {
    fn new(cap: usize) -> Self {
        Self {
            gaps: Vec::with_capacity(cap),
            first: [INVALID; 9],
            last: [INVALID; 9],
        }
    }

    fn add(&mut self, offset: u32, len: u32) {
        if len == 0 || len > 9 {
            panic!("Invalid gap length: {len}");
        }
        let last = self.gaps.len();
        self.gaps.push(Gap {
            offset,
            len,
            links: [(INVALID, INVALID); 9],
        });

        for g in 0..len as usize {
            if self.first[g] == INVALID {
                self.first[g] = last as u32;
            } else {
                self.gaps[self.last[g] as usize].links[g].1 = last as u32;
                self.gaps[last].links[g].0 = self.last[g];
            }
            self.last[g] = last as u32;
        }
    }

    fn get(&mut self, offset: u32, len: u32) -> Option<u32> {
        let idx = self.first[len as usize - 1];
        if idx == INVALID || self.gaps[idx as usize].offset >= offset {
            return None;
        }

        let idx = idx as usize;
        let old_offset = self.gaps[idx].offset;
        let old_len = self.gaps[idx].len;
        self.gaps[idx].offset += len;
        self.gaps[idx].len -= len;

        for g in self.gaps[idx].len as usize..old_len as usize {
            let (prev, next) = self.gaps[idx].links[g];
            if prev != INVALID {
                self.gaps[prev as usize].links[g].1 = next;
            } else {
                self.first[g] = next;
            }

            if next != INVALID {
                self.gaps[next as usize].links[g].0 = prev;
            } else {
                self.last[g] = prev;
            }
        }

        Some(old_offset)
    }
}

fn main() -> Result<()> {
    let input = open_input("day9")?;
    let data = {
        let mut b = input.bytes();
        while b.len() > 0 && is_nl(b[b.len() - 1]) {
            b = &b[..b.len() - 1];
        }
        b
    };

    let mut total1 = 0;

    let mut front = 0;
    let mut back = data.len() - 1;
    let mut write = 0;
    let mut last_len = (data[back] - b'0') as u32;

    while front < back {
        let f = (data[front] - b'0') as u32;
        // println!("{f} * {}", front / 2);
        total1 += sum(write, write + f - 1) * (front / 2) as uresult;
        write += f;

        front += 1;
        let mut gap = (data[front] - b'0') as u32;
        while gap > 0 {
            let min = last_len.min(gap);
            // println!("{min} * {}", back / 2);
            total1 += sum(write, write + min - 1) * (back / 2) as uresult;
            last_len -= min;
            gap -= min;
            write += min;

            if last_len == 0 {
                back -= 2;
                last_len = (data[back] - b'0') as u32;
            }
        }

        front += 1;
    }

    if last_len > 0 {
        // println!("{last_len} * {}", back / 2);
        total1 += sum(write, write + last_len - 1) * (back / 2) as uresult;
    }

    let mut gap_list = GapList::new(data.len() / 2 + 1);
    let mut orig_pos = vec![0; (data.len() + 1) / 2];
    let mut offset = 0;

    for (idx, e) in data.chunks(2).enumerate() {
        let l = (e[0] - b'0') as u32;
        orig_pos[idx as usize] = offset;
        offset += l;
        if e.len() > 1 {
            let g = (e[1] - b'0') as u32;
            if g > 0 {
                gap_list.add(offset, g);
                offset += g;
            }
        }
    }

    let mut total2 = 0;

    for (idx, &len) in data.iter().step_by(2).enumerate().rev() {
        let len = (len - b'0') as u32;
        if let Some(offset) = gap_list.get(orig_pos[idx], len) {
            // println!("{idx}*{len} at {offset} [moved]");
            total2 += sum(offset, offset + len - 1) * idx as uresult;
        } else {
            // println!("{idx}*{len} at {}", orig_pos[idx]);
            total2 += sum(orig_pos[idx], orig_pos[idx] + len - 1) * idx as uresult;
        }
    }

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
