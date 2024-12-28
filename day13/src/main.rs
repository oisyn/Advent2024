#![allow(dead_code)]

use util::*;

#[aoc_day]
fn solve(input: Input) -> impl AocResult {
    let mut lines = input.lines();

    let mut total1 = 0;
    let mut total2 = 0;
    loop {
        let Some(a) = lines.next() else {
            break;
        };
        let b = lines.next().unwrap();
        let prize = lines.next().unwrap();

        let mut a = Parser::new(a);
        let a = coord(
            a.expect("Button A: X+").parse::<i32>().unwrap(),
            a.expect(", Y+").parse::<i32>().unwrap(),
        );

        let mut b = Parser::new(b);
        let b = coord(
            b.expect("Button B: X+").parse::<i32>().unwrap(),
            b.expect(", Y+").parse::<i32>().unwrap(),
        );

        let mut prize = Parser::new(prize);
        let prize = coord(
            prize.expect("Prize: X=").parse::<i32>().unwrap(),
            prize.expect(", Y=").parse::<i32>().unwrap(),
        );

        /*
           Matrix = |a.x b.x|
                    |a.y b.y|

           inverse= |b.y -b.x|  /
                    |-a.y a.x| /  det

           result = (|b.y -a.y| * prize.x + |-b.x a.x| * prize.y) / det
        */

        let det = (a.x * b.y) - (a.y * b.x);
        let invx = coord(b.y, -a.y);
        let invy = coord(-b.x, a.x);
        let mut result = invx * prize.x + invy * prize.y;
        if result % det == coord(0, 0) {
            result /= det;
            let cost = 3 * result.x + result.y;
            total1 += cost;
        }

        let det = det as i64;
        let prize = prize.to::<i64>() + coord(10000000000000, 10000000000000);
        let invx = invx.to::<i64>();
        let invy = invy.to::<i64>();
        let mut result = invx * prize.x + invy * prize.y;
        if result % det == coord(0, 0) {
            result /= det;
            let cost = 3 * result.x + result.y;
            total2 += cost;
        }

        lines.next();
    }

    (total1, total2)
}
