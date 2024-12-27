#![allow(dead_code)]

use anyhow::Result;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day25")?;

    let mut locks = Vec::with_capacity(100);
    let mut keys = Vec::with_capacity(100);

    for p in input.paragraphs() {
        let field = FieldView::from(p.as_bytes());

        if field[0] == b'#' {
            let sig = [0, 1, 2, 3, 4]
                .map(|i| field.col(i).into_iter().take_while(|&&b| b == b'#').count() as u8 - 1);
            locks.push(sig);
        } else {
            let sig = [0, 1, 2, 3, 4].map(|i| {
                field
                    .col(i)
                    .into_iter()
                    .rev()
                    .take_while(|&&b| b == b'#')
                    .count() as u8
                    - 1
            });
            keys.push(sig);
        }
    }

    let mut total1 = 0;

    for l in &locks {
        for k in &keys {
            total1 += l.iter().zip(k.iter()).all(|(&l, &k)| l + k <= 5) as u32;
        }
    }

    let total2 = 0;

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
