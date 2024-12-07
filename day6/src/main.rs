use anyhow::Result;
use std::collections::HashSet;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day6")?;
    let field = BorderedFieldView::new(FieldView::from(&input), b' ');

    const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut dir = 0;
    let pos = field.from_offset(field.data().iter().position(|&c| c == b'^').unwrap());
    let mut pos = (pos.0 as i32, pos.1 as i32);
    let start_pos = pos;
    let mut visited = HashSet::with_capacity(1000);
    let mut turns = HashSet::with_capacity(1000);

    visited.insert(pos);

    let mut is_looping = |wall_pos: (i32, i32), mut dir: usize| {
        turns.clear();
        let mut pos = (wall_pos.0 - DIRS[dir].0, wall_pos.1 - DIRS[dir].1);
        dir = (dir + 1) & 3;
        turns.insert((pos, dir));

        loop {
            let newpos = (pos.0 + DIRS[dir].0, pos.1 + DIRS[dir].1);
            match if newpos == wall_pos {
                b'#'
            } else {
                *field.get(newpos.0 as usize, newpos.1 as usize)
            } {
                b'#' => {
                    dir = (dir + 1) & 3;
                    if !turns.insert((newpos, dir)) {
                        return true;
                    }
                }
                b' ' => return false,
                _ => pos = newpos,
            }
        }
    };

    let mut total2 = 0;

    loop {
        let newpos = (pos.0 + DIRS[dir].0, pos.1 + DIRS[dir].1);
        match field.get(newpos.0 as usize, newpos.1 as usize) {
            b'#' => {
                dir = (dir + 1) & 3;
            }
            b' ' => break,
            _ => {
                if visited.insert(newpos) && newpos != start_pos && is_looping(newpos, dir) {
                    total2 += 1;
                }

                pos = newpos;
            }
        }
    }

    let total1 = visited.len();

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
