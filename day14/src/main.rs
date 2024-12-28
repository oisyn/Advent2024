#![allow(dead_code)]

use util::*;

#[aoc_day]
fn solve(input: Input) -> impl AocResult {
    let size = match input.is_example() {
        false => coord(101, 103),
        true => coord(11, 7),
    };
    let mid = size / 2;

    const SECONDS: i32 = 100;

    let mut quadrants = [0; 4];
    let mut robots = Vec::with_capacity(500);
    for l in input.lines() {
        let mut p = Parser::new(l);
        let pos = coord(
            p.expect("p=").parse::<i32>().unwrap(),
            p.expect(",").parse::<i32>().unwrap(),
        );
        let v = coord(
            p.expect(" v=").parse::<i32>().unwrap(),
            p.expect(",").parse::<i32>().unwrap(),
        );
        robots.push((pos, v));

        let new_pos = pos + v * SECONDS;
        let new_pos = coord(new_pos.x.rem_euclid(size.x), new_pos.y.rem_euclid(size.y));
        match new_pos.tuple() {
            (x, y) if x < mid.x && y < mid.y => quadrants[0] += 1,
            (x, y) if x > mid.x && y < mid.y => quadrants[1] += 1,
            (x, y) if x < mid.x && y > mid.y => quadrants[2] += 1,
            (x, y) if x > mid.x && y > mid.y => quadrants[3] += 1,
            _ => {}
        }
    }
    let total1 = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];

    let mut minscore = coord(i32::MAX, i32::MAX);
    let mut minpos = coord(0, 0);
    for i in 0..size.x.max(size.y) {
        let mut quadrants = [0; 4];
        for r in &robots {
            let pos = r.0 + r.1 * i;
            let pos = coord(pos.x.rem_euclid(size.x), pos.y.rem_euclid(size.y));
            match pos.tuple() {
                (x, y) if x < mid.x && y < mid.y => quadrants[0] += 1,
                (x, y) if x > mid.x && y < mid.y => quadrants[1] += 1,
                (x, y) if x < mid.x && y > mid.y => quadrants[2] += 1,
                (x, y) if x > mid.x && y > mid.y => quadrants[3] += 1,
                _ => {}
            }
        }
        let score = coord(
            (quadrants[0] + quadrants[2]) * (quadrants[1] + quadrants[3]),
            (quadrants[0] + quadrants[1]) * (quadrants[2] + quadrants[3]),
        );
        if minscore.x > score.x {
            minscore.x = score.x;
            minpos.x = i;
        }
        if minscore.y > score.y {
            minscore.y = score.y;
            minpos.y = i;
        }
    }

    let (_, x, y) = extended_euclidian(size.x, size.y);
    let total2 = (minpos.x * y * size.y + minpos.y * x * size.x) % (size.x * size.y);

    (total1, total2)
}
