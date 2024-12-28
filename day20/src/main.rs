#![allow(dead_code)]

use util::*;

#[aoc_day]
fn solve(input: Input) -> impl AocResult {
    let field = FieldView::from(&input);

    let start =
        field.coord_from_offset::<i32>(field.data().iter().position(|&c| c == b'S').unwrap());
    let end = field.coord_from_offset::<i32>(field.data().iter().position(|&c| c == b'E').unwrap());
    let mut path = Vec::with_capacity(10000);
    let mut lengths =
        FieldMutView::create_with_value(-1, field.width(), field.width(), field.height());

    let mut pos = start;
    let mut dir = coord(1, 0);
    let mut total_len = 0;
    'path: while pos != end {
        path.push(pos);
        lengths[pos] = total_len;
        total_len += 1;

        for d in [dir, dir.turn_left(), dir.turn_right(), -dir] {
            let next = pos + d;
            if field[next] != b'#' {
                pos = next;
                dir = d;
                continue 'path;
            }
        }
        unreachable!();
    }
    lengths[end] = total_len;

    let mut total1 = 0;
    let mut total2 = 0;
    for &p in &path {
        let start_cut = lengths[p];
        for n in p.neighbors4() {
            if field[n] != b'#' {
                continue;
            }

            let n2 = n + n - p;
            if !field.in_bounds_coord(n2) {
                continue;
            }

            if lengths[n2] - start_cut - 2 >= 100 {
                total1 += 1;
            }
        }

        for y in 0.max(p.y - 20)..(field.height() as i32).min(p.y + 21) {
            let y_cost = y.abs_diff(p.y) as i32;
            let x_count = 20 - y_cost as i32;
            for x in 0.max(p.x - x_count)..(field.width() as i32).min(p.x + x_count + 1) {
                let cost = x.abs_diff(p.x) as i32 + y_cost;
                if lengths[coord(x, y)] - start_cut - cost >= 100 {
                    total2 += 1;
                }
            }
        }
    }

    (total1, total2)
}
