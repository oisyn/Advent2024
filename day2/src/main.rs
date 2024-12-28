use itertools::*;
use util::*;

#[aoc_day]
fn solve(input: Input) -> impl AocResult {

    let mut total1 = 0;
    let mut total2 = 0;

    for l in input.lines() {
        let v = Parser::new(l).parse_iter(" ").collect::<Vec<i32>>();

        fn check(iter: impl Iterator<Item = i32>) -> bool {
            let mut order = None;

            for (a, b) in iter.into_iter().tuple_windows() {
                match order {
                    None => {
                        order = Some(a.cmp(&b));
                    }
                    Some(ord) => {
                        if a.cmp(&b) != ord {
                            return false;
                        }
                    }
                };
                if !(1..=3).contains(&a.abs_diff(b)) {
                    return false;
                }
            }
            true
        }

        let pass = check(v.iter().copied());
        if pass {
            total1 += 1;
            total2 += 1;
        } else {
            for i in 0..v.len() {
                if check(v.iter().take(i).chain(v.iter().skip(i + 1)).copied()) {
                    total2 += 1;
                    break;
                }
            }
        }
    }

    (total1, total2)
}
