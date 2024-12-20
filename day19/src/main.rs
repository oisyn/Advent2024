#![allow(dead_code)]

use anyhow::Result;
use util::*;

const fn generate_lookup() -> [u8; 128] {
    let mut lookup = [0; 128];
    lookup[b'r' as usize] = 0;
    lookup[b'g' as usize] = 1;
    lookup[b'b' as usize] = 2;
    lookup[b'u' as usize] = 3;
    lookup[b'w' as usize] = 4;
    lookup
}

const LOOKUP: [u8; 128] = generate_lookup();
const NUM_NODES: usize = 6;
const END: usize = 5;
const INVALID: u32 = u32::MAX;

struct Trie {
    nodes: Vec<[u32; NUM_NODES]>,
}

impl Trie {
    fn new() -> Self {
        let mut nodes = Vec::new();
        nodes.push([INVALID, INVALID, INVALID, INVALID, INVALID, 0]);
        Self { nodes }
    }

    fn insert(&mut self, s: &[u8]) {
        let mut node = 0;
        for &c in s {
            let idx = LOOKUP[c as usize] as usize;
            let mut next = self.nodes[node][idx];
            if next == INVALID {
                next = self.nodes.len() as u32;
                self.nodes[node][idx] = next;
                self.nodes.push([INVALID; NUM_NODES]);
            }
            node = next as usize;
        }
        self.nodes[node][END] = 0;
    }

    fn pass_impl(&self, len_checked: &mut u64, s: &[u8]) -> bool {
        if *len_checked & (1_u64 << s.len()) != 0 {
            return false;
        }
        *len_checked |= 1_u64 << s.len();

        let mut node = 0;
        for i in 0..s.len() {
            if node != 0 && self.nodes[node][END] == 0 && self.pass_impl(len_checked, &s[i..]) {
                return true;
            }
            let idx = LOOKUP[s[i] as usize] as usize;
            let next = self.nodes[node][idx];
            if next == INVALID {
                return false;
            }
            node = next as usize;
        }
        self.nodes[node][END] == 0
    }

    fn pass(&self, s: &[u8]) -> bool {
        self.pass_impl(&mut 0, s)
    }
}

fn main() -> Result<()> {
    let input = open_input("day19")?;
    let mut lines = input.lines();
    let mut p = Parser::new(lines.next().unwrap());
    let mut trie = Trie::new();
    while !p.at_end() {
        let towel = p.take_while(|c| c != b',').as_bytes();
        p.skip(2);
        trie.insert(towel);
    }

    lines.next();
    let mut total1 = 0;
    let total2 = 0;
    for l in lines {
        total1 += trie.pass(l.as_bytes()) as u32;
    }

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
