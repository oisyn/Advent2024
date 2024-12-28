use util::*;

fn next_pow10(v: usize) -> usize {
    const POW10S: &[usize] = &[10, 100, 1000, 10000, 100000];
    for &n in POW10S {
        if n > v {
            return n;
        }
    }
    panic!("Unexpectedly large number: {v}")
}

fn check(target: usize, n: &[usize], do_concat: bool) -> bool {
    if n.len() == 1 {
        return target == n[0];
    }

    let (&last, n) = n.split_last().unwrap();
    if target == 0 && last == 0 {
        return true;
    }
    if target > 0 && target < last {
        return false;
    }
    if last != 0 && target % last == 0 && check(target / last, n, do_concat) {
        return true;
    }
    if do_concat {
        let pow10 = next_pow10(last);
        if target % pow10 == last && check(target / pow10, n, true) {
            return true;
        }
    }
    target >= last && check(target - last, n, do_concat)
}

#[aoc_day]
fn solve(input: Input) -> impl AocResult {
    let mut n = Vec::with_capacity(20);

    let mut total1 = 0;
    let mut total2 = 0;

    for l in input.lines() {
        let mut p = Parser::new(l);
        let target = p.parse().unwrap();
        p.expect(": ");
        n.clear();
        n.extend(p.parse_iter::<usize>(" "));

        if check(target, &n, false) {
            total1 += target;
            total2 += target;
        } else if check(target, &n, true) {
            total2 += target;
        }
    }

    (total1, total2)
}
