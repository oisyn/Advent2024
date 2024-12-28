use std::collections::{HashMap, HashSet};
use util::*;

#[aoc_day]
fn solve(input: Input) -> impl AocResult {
    let field = FieldView::from(&input);

    let width = field.width() as i32;
    let height = field.height() as i32;
    let is_valid =
        |coord: Coord<i32>| (0..width).contains(&coord.x) && (0..height).contains(&coord.y);

    let mut map: HashMap<u8, Vec<Coord<i32>>> = HashMap::with_capacity(1000);
    for y in 0..height {
        for x in 0..width {
            let c = field[(x, y)];
            if c == b'.' {
                continue;
            }

            map.entry(c).or_default().push(coord(x, y));
        }
    }

    let mut found = HashSet::with_capacity(1000);

    for (&k, v) in &map {
        for i in 0..v.len() - 1 {
            for j in i + 1..v.len() {
                let v0 = v[i];
                let v1 = v[j];
                let diff = v1 - v0;
                let p0 = v0 - diff;
                if is_valid(p0) && field[p0] != k {
                    found.insert(p0);
                }
                let p1 = v1 + diff;
                if is_valid(p1) && field[p1] != k {
                    found.insert(p1);
                }

                // Apparently this never triggers
                /*
                if diff % 3 == coord(0, 0) {
                    let diff3 = diff / 3;
                    let p2 = v0 + diff3;
                    if field[p2] != k {
                        found.insert(p2);
                    }
                    let p3 = v1 - diff3;
                    if field[p3] != k {
                        found.insert(p3);
                    }
                }
                */
            }
        }
    }

    let total1 = found.len();

    for (&k, v) in &map {
        for i in 0..v.len() - 1 {
            for j in i + 1..v.len() {
                let v0 = v[i];
                let v1 = v[j];
                let diff = v1 - v0;

                // Apparently this never triggers
                //let diff = diff / gcd(diff.x, diff.y).abs();

                found.insert(v0);
                found.insert(v1);

                let mut p0 = v0 - diff * 2;
                while is_valid(p0) {
                    if field[p0] != k {
                        found.insert(p0);
                    }
                    p0 -= diff;
                }

                let mut p1 = v1 + diff * 2;
                while is_valid(p1) {
                    if field[p1] != k {
                        found.insert(p1);
                    }
                    p1 += diff;
                }
            }
        }
    }

    let total2 = found.len();

    (total1, total2)
}
