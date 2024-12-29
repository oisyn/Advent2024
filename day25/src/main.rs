#![allow(dead_code)]

use util::*;

#[aoc_day]
fn solve(input: Input) -> impl AocResult {
    let mut keys = Vec::with_capacity(100);

    const STRIDES: [usize; 5] = [6 * 6 * 6 * 6, 6 * 6 * 6, 6 * 6, 6, 1];
    fn offset(sig: [u8; 5]) -> usize {
        sig[0] as usize * STRIDES[0]
            + sig[1] as usize * STRIDES[1]
            + sig[2] as usize * STRIDES[2]
            + sig[3] as usize * STRIDES[3]
            + sig[4] as usize
    }

    let mut counts = vec![0_u32; 6 * 6 * 6 * 6 * 6];

    for p in input.paragraphs() {
        let field = FieldView::from(p.as_bytes());

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
            counts[o] += 1;
        } else {
            keys.push(o);
        }
    }

    for axis in 0..5 {
        let outer = STRIDES[4 - axis];
        let inner = STRIDES[axis];
        for o in 0..outer {
            for i in 0..inner {
                let mut offset = o * inner * 6 + i;
                for _ in 1..6 {
                    counts[offset + inner] += counts[offset];
                    offset += inner;
                }
            }
        }
    }

    let total1 = keys.into_iter().map(|k| counts[k]).sum::<u32>();

    (total1, 0)
}
