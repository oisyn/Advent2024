use anyhow::Result;
use std::collections::{HashSet, VecDeque};
use util::*;

const DIRS: [Coord<i32>; 4] = [coord(1, 0), coord(0, 1), coord(-1, 0), coord(0, -1)];

fn main() -> Result<()> {
    let input = open_input("day10")?;
    let field = BorderedFieldView::new(FieldView::from(&input), b' ');

    let mut total1 = 0;

    let mut done = HashSet::with_capacity(1000);
    let mut queue = Vec::with_capacity(1000);

    for (offset, _) in field.data().iter().enumerate().filter(|(_, &c)| c == b'0') {
        done.clear();
        queue.push(field.coord_from_offset::<i32>(offset));

        while let Some(coord) = queue.pop() {
            let c = field[coord] + 1;
            for d in DIRS {
                let next = coord + d;
                if field[next] == c && done.insert(next) {
                    if c == b'9' {
                        total1 += 1;
                    } else {
                        queue.push(next);
                    }
                }
            }
        }
    }

    let (width, height) = (field.width(), field.height());
    let off = |c: Coord<i32>| c.y as usize * width + c.x as usize;

    let mut reachable = vec![0; width * height];
    let mut total2 = 0;
    let mut queue = field
        .data()
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == b'9')
        .map(|(offset, _)| field.coord_from_offset::<i32>(offset))
        .collect::<VecDeque<_>>();
    done.clear();

    while let Some(coord) = queue.pop_front() {
        let c = field[coord];
        let num = if c == b'9' {
            reachable[off(coord)] = 1;
            1
        } else {
            reachable[off(coord)]
        };

        let c = c - 1;
        for d in DIRS {
            let next = coord + d;
            if field[next] == c {
                reachable[off(next)] += num;
                if c == b'0' {
                    total2 += num;
                } else if done.insert(next) {
                    queue.push_back(next);
                }
            }
        }
    }

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
