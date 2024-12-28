#![allow(dead_code)]

use core::slice;
use itertools::*;
use util::*;

#[aoc_day]
fn solve(input: Input) -> impl AocResult {
    let (field_in, code_in) = input.paragraphs().collect_tuple().unwrap();
    let mut field_vec = Vec::from(field_in.as_bytes());
    let mut field = FieldMutView::from(field_vec.as_mut_slice());

    let robot_start =
        field.coord_from_offset::<i32>(field.data().iter().position(|&c| c == b'@').unwrap());
    let mut robot = robot_start;
    field[robot] = b'.';

    for &c in code_in.as_bytes() {
        let dir = match c {
            b'^' => coord(0, -1),
            b'v' => coord(0, 1),
            b'<' => coord(-1, 0),
            b'>' => coord(1, 0),
            _ => {
                continue;
            }
        };

        let mut pos = robot + dir;
        let mut pushed = false;
        while field[pos] == b'O' {
            pos += dir;
            pushed = true;
        }

        if field[pos] == b'#' {
            continue;
        }

        if pushed {
            field[pos] = b'O';
            field[robot + dir] = b'.';
        }

        robot += dir;
        field[robot] = b'.';
    }

    let total1 = field
        .data()
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == b'O')
        .map(|(i, _)| {
            let pos = field.coord_from_offset::<i32>(i);
            (pos.y * 100 + pos.x) as u64
        })
        .sum::<u64>();

    field_vec = field_in
        .as_bytes()
        .iter()
        .flat_map(|c| match c {
            b'#' => [b'#', b'#'].as_slice(),
            b'.' => [b'.', b'.'].as_slice(),
            b'O' => [b'[', b']'].as_slice(),
            b'@' => [b'.', b'.'].as_slice(),
            c => slice::from_ref(c),
        })
        .copied()
        .collect();
    let mut field = FieldMutView::from(field_vec.as_mut_slice());

    let mut robot = coord(robot_start.x * 2, robot_start.y);
    let mut checked_vec = vec![false; field.width() * field.height()];
    let mut checked = FieldMutView::from_ref(
        &mut checked_vec,
        field.width(),
        field.width(),
        field.height(),
    );
    let mut queue = Vec::with_capacity(1000);
    'outer: for &c in code_in.as_bytes() {
        let dir = match c {
            b'^' => coord(0, -1),
            b'v' => coord(0, 1),
            b'<' => coord(-1, 0),
            b'>' => coord(1, 0),
            _ => {
                continue;
            }
        };

        // field[robot] = b'@';
        // println!(
        //     "move {}\n{}",
        //     to_str(slice::from_ref(&c)),
        //     to_str(field.data())
        // );
        // std::io::stdin().read_line(&mut String::new()).unwrap();
        // field[robot] = b'.';

        let mut pos = robot + dir;
        if dir.y == 0 {
            let mut pushed = false;
            while field[pos] == b'[' || field[pos] == b']' {
                pos += 2 * dir;
                pushed = true;
            }

            if field[pos] == b'#' {
                continue;
            }

            if pushed {
                if dir.x < 0 {
                    for x in (pos.x..robot.x - 1).step_by(2) {
                        let off = field.offset(x, pos.y);
                        field[off] = b'[';
                        field[off + 1] = b']';
                    }
                } else {
                    for x in (robot.x + 2..pos.x).step_by(2) {
                        let off = field.offset(x, pos.y);
                        field[off] = b'[';
                        field[off + 1] = b']';
                    }
                }
                field[robot + dir] = b'.';
            }
        } else {
            'ok: {
                for &p in &queue {
                    checked[p] = false;
                }
                queue.clear();
                match field[pos] {
                    b'#' => {
                        continue 'outer;
                    }
                    b'.' => {
                        break 'ok;
                    }
                    b'[' => {
                        queue.push(pos);
                    }
                    b']' => {
                        queue.push(pos.left());
                    }
                    _ => unreachable!(),
                }

                for i in 0.. {
                    if i == queue.len() {
                        break;
                    }
                    let pos = queue[i] + dir;
                    match (field[pos], field[pos.right()]) {
                        (b'#', _) | (_, b'#') => {
                            continue 'outer;
                        }
                        (b'.', b'.') => {
                            continue;
                        }
                        (c0, c1) => {
                            if c0 == b'[' {
                                queue.push(pos);
                            } else if c0 == b']' && !checked[pos.left()].exchange(true) {
                                queue.push(pos.left());
                            }

                            if c1 == b'[' && !checked[pos.right()].exchange(true) {
                                queue.push(pos.right());
                            }
                        }
                    }
                }

                for &pos in queue.iter().rev() {
                    field[pos] = b'.';
                    field[pos.right()] = b'.';
                    let pos = pos + dir;
                    field[pos] = b'[';
                    field[pos.right()] = b']';
                }
            }
        }

        robot += dir;
    }

    let total2 = field
        .data()
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == b'[')
        .map(|(i, _)| {
            let pos = field.coord_from_offset::<i32>(i);
            (pos.y * 100 + pos.x) as u64
        })
        .sum::<u64>();

    (total1, total2)
}
