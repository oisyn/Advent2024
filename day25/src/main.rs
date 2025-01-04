#![allow(dead_code)]

use util::*;

#[aoc_day]
fn solve(input: Input) -> impl AocResult {
    const STRIDES: [usize; 5] = [6 * 6 * 6 * 6, 6 * 6 * 6, 6 * 6, 6, 1];
    fn offset(sig: [u8; 5]) -> usize {
        sig[0] as usize * STRIDES[0]
            + sig[1] as usize * STRIDES[1]
            + sig[2] as usize * STRIDES[2]
            + sig[3] as usize * STRIDES[3]
            + sig[4] as usize
    }

    let mut locks = vec![0_u32; 6 * 6 * 6 * 6 * 6];
    let mut keys = vec![0_u32; 6 * 6 * 6 * 6 * 6];
    let mut total1 = 0;
    let nl_size = input.nl_size();

    for p in input.bytes().chunks(7 * (5 + nl_size) + nl_size) {
        let field = FieldView::new(p, 5, 5 + nl_size, 7);

        let c = field[0];
        let sig = [0, 1, 2, 3, 4].map(|i| {
            field
                .col(i)
                .into_iter()
                .skip(1)
                .take_while(|&&b| b == c)
                .count() as u8
        });

        let o = offset(sig);

        if c == b'#' {
            locks[o] += 1;
            total1 += keys[o] as u64;
        } else {
            keys[o] += 1;
            total1 += locks[o] as u64;
        }
    }

    for axis in 0..5 {
        let outer = STRIDES[4 - axis];
        let inner = STRIDES[axis];
        for o in 0..outer {
            for i in 0..inner {
                let mut offset = o * inner * 6 + i;
                for _ in 1..6 {
                    let p = locks[offset];
                    locks[offset + inner] += p;
                    total1 += (p * keys[offset + inner]) as u64;
                    offset += inner;
                }
            }
        }
    }

    (total1, 0)
}
