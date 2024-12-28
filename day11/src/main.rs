use std::collections::HashMap;

use util::*;

fn split(n: u64) -> Option<(u64, u64)> {
    // split n into two parts if it has an even number of digits
    let mut d = 10;
    let mut div = 10;
    let mut num = 1;
    while d <= n {
        d *= 10;
        if num & 1 == 0 {
            div *= 10;
        }
        num += 1;
    }

    if num & 1 != 0 {
        None
    } else {
        Some((n / div, n % div))
    }
}

fn num_stones_from_number(n: u64, steps_left: i32, mem: &mut HashMap<(u64, i32), u64>) -> u64 {
    if steps_left == 0 {
        return 1;
    }

    if let Some(&v) = mem.get(&(n, steps_left)) {
        return v;
    }

    let num = if n == 0 {
        num_stones_from_number(1, steps_left - 1, mem)
    } else if let Some((a, b)) = split(n) {
        num_stones_from_number(a, steps_left - 1, mem)
            + num_stones_from_number(b, steps_left - 1, mem)
    } else {
        num_stones_from_number(n * 2024, steps_left - 1, mem)
    };

    mem.insert((n, steps_left), num);
    num
}

const STEPS1: i32 = 25;
const STEPS2: i32 = 75;

#[aoc_day]
fn solve(input: Input) -> impl AocResult {
    let mut mem = HashMap::with_capacity(100000);
    let (total1, total2) = Parser::new(input.str())
        .parse_iter::<u64>(" ")
        .map(|n| {
            (
                num_stones_from_number(n, STEPS1, &mut mem),
                num_stones_from_number(n, STEPS2, &mut mem),
            )
        })
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));

    println!("{:?}", mem.len());

    (total1, total2)
}
