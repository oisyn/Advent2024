use anyhow::Result;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day1")?;

    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    for l in input.lines() {
        let mut p = Parser::new(l);

        while !p.at_end() {
            left.push(p.parse::<i32>().unwrap());
            p.expect("   ");
            right.push(p.parse::<i32>().unwrap());
        }
    }

    left.sort_unstable();
    right.sort_unstable();
    let mut total1: isize = 0;
    for (&l, &r) in left.iter().zip(&right) {
        total1 += (l - r).abs() as isize;
    }

    let mut total2: isize = 0;
    let mut rest = &right[..];
    let mut last = (-1, 0);
    for &l in &left {
        if l == last.0 {
            total2 += last.1;
            continue;
        }

        if rest.is_empty() {
            break;
        }

        use std::cmp::Ordering::*;
        let start = rest
            .binary_search_by(|&v| match v.cmp(&l) {
                Equal => Greater,
                ord => ord,
            })
            .unwrap_err();

        let mut end = start;
        while end < rest.len() && rest[end] == l {
            end += 1;
        }

        last = (l, (l * (end - start) as i32) as isize);
        total2 += last.1;
        rest = &rest[end..];
    }

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
