#![allow(dead_code)]

use anyhow::Result;
use util::*;

const BUTTON_UP: u8 = 0;
const BUTTON_DOWN: u8 = 1;
const BUTTON_LEFT: u8 = 2;
const BUTTON_RIGHT: u8 = 3;
const BUTTON_A: u8 = 10;

const NUMBER_PAD: [Coord<i32>; 11] = [
    coord(1, 3), // 0
    coord(0, 2), // 1
    coord(1, 2), // 2
    coord(2, 2), // 3
    coord(0, 1), // 4
    coord(1, 1), // 5
    coord(2, 1), // 6
    coord(0, 0), // 7
    coord(1, 0), // 8
    coord(2, 0), // 9
    coord(2, 3), // A
];

const DIRECTION_PAD: [Coord<i32>; 11] = [
    coord(1, 0), // Up
    coord(1, 1), // Down
    coord(0, 1), // Left
    coord(2, 1), // Right
    coord(0, 0),
    coord(0, 0),
    coord(0, 0),
    coord(0, 0),
    coord(0, 0),
    coord(0, 0),
    coord(2, 0), // A
];

const DIRECTION_CHARS: &[u8; 11] = b"^v<>      A";

type StateLut = [[u64; 11]; 11];

fn press_number(key: u8, prev: &mut u8, lut: &mut [StateLut]) -> u64 {
    press(key, prev, &NUMBER_PAD, lut)
}

fn press_direction(key: u8, prev: &mut u8, lut: &mut [StateLut]) -> u64 {
    press(key, prev, &DIRECTION_PAD, lut)
}

fn press(key: u8, prev: &mut u8, pad: &[Coord<i32>], lut: &mut [StateLut]) -> u64 {
    let (this_lut, lut) = lut.split_first_mut().unwrap();

    if this_lut[*prev as usize][key as usize] > 0 {
        let total = this_lut[*prev as usize][key as usize];
        *prev = key;
        return total;
    }

    let mut total = 0;
    let a_pos = pad[BUTTON_A as usize];
    let pos = pad[*prev as usize];
    let new_pos = pad[key as usize];
    let diff = new_pos - pos;

    if lut.is_empty() {
        let total = (diff.x.abs() + diff.y.abs() + 1) as u64;
        this_lut[*prev as usize][key as usize] = total;
        *prev = key;
        return total;
    }

    let mut cur = BUTTON_A;

    if diff.x < 0 && !(new_pos.x == 0 && pos.y == a_pos.y) {
        for _ in 0..-diff.x {
            total += press_direction(BUTTON_LEFT, &mut cur, lut);
        }
    }
    if diff.x > 0 && (pos.x == 0 && new_pos.y == a_pos.y) {
        for _ in 0..diff.x {
            total += press_direction(BUTTON_RIGHT, &mut cur, lut);
        }
    }

    if diff.y > 0 {
        for _ in 0..diff.y {
            total += press_direction(BUTTON_DOWN, &mut cur, lut);
        }
    } else if diff.y < 0 {
        for _ in 0..-diff.y {
            total += press_direction(BUTTON_UP, &mut cur, lut);
        }
    }

    if diff.x > 0 && !(pos.x == 0 && new_pos.y == a_pos.y) {
        for _ in 0..diff.x {
            total += press_direction(BUTTON_RIGHT, &mut cur, lut);
        }
    }
    if diff.x < 0 && (new_pos.x == 0 && pos.y == a_pos.y) {
        for _ in 0..-diff.x {
            total += press_direction(BUTTON_LEFT, &mut cur, lut);
        }
    }

    total += press_direction(BUTTON_A, &mut cur, lut);
    this_lut[*prev as usize][key as usize] = total;
    *prev = key;

    total
}

fn dir_str(seq: &[u8]) -> String {
    let mut s = String::with_capacity(seq.len());
    for &c in seq {
        s.push(DIRECTION_CHARS[c as usize] as char);
    }
    s
}

fn main() -> Result<()> {
    let input = open_input("day21")?;
    let mut total1 = 0;
    let mut total2 = 0;

    let mut lut1 = [StateLut::default(); 3];
    let mut lut2 = [StateLut::default(); 26];

    for l in input.lines() {
        let b = l.as_bytes();
        let mut seq = [BUTTON_A; 4];
        let mut weight = 0;
        for i in 0..3 {
            seq[i] = b[i] - b'0';
            weight = weight * 10 + seq[i] as u64;
        }

        let mut len = 0;
        let mut cur = BUTTON_A;
        for &key in &seq {
            len += press_number(key, &mut cur, &mut lut1);
        }
        println!("{l}: {len}");

        total1 += weight * len;

        let mut len = 0;
        let mut cur = BUTTON_A;
        for &key in &seq {
            len += press_number(key, &mut cur, &mut lut2);
        }

        total2 += weight * len;
    }

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
