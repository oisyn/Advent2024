#![allow(dead_code)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use anyhow::Result;
use util::*;

const DIRS: [Coord<i32>; 4] = [coord(1, 0), coord(0, 1), coord(-1, 0), coord(0, -1)];
const fn dir_coord(i: u32) -> Coord<i32> {
    DIRS[i as usize]
}

const COST_MASK: u32 = 0x0fff_ffff;
const FLAG_SHIFT: u32 = 28;
const BASE_FLAG: u32 = 1 << FLAG_SHIFT;
const FLAG_MASK: u32 = 0xf << FLAG_SHIFT;

#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct CoordAndDir(u32);

impl CoordAndDir {
    fn new(pos: Coord<i32>, dir: u32) -> Self {
        CoordAndDir(
            (pos.x as u32 & 0x7fff) | (pos.y as u32 & 0x7fff) << 15 | (dir as u32 & 0x3) << 30,
        )
    }
}

impl From<(Coord<i32>, u32)> for CoordAndDir {
    fn from((pos, dir): (Coord<i32>, u32)) -> Self {
        CoordAndDir::new(pos, dir)
    }
}

impl From<CoordAndDir> for (Coord<i32>, u32) {
    fn from(c: CoordAndDir) -> Self {
        (
            coord((c.0 as i32) << 17 >> 17, (c.0 as i32) << 2 >> 17),
            c.0 >> 30,
        )
    }
}

fn main() -> Result<()> {
    let input = open_input("day16")?;
    let field = FieldView::from(&input);

    let start =
        field.coord_from_offset::<i32>(field.data().iter().position(|&c| c == b'S').unwrap());
    let end = field.coord_from_offset::<i32>(field.data().iter().position(|&c| c == b'E').unwrap());

    let mut queue = BinaryHeap::with_capacity(10000);
    let mut reverse =
        FieldMutView::create_with_value([0_u32; 4], field.width(), field.width(), field.height());

    let mut cost_map = HashMap::with_capacity(100);
    cost_map.insert(0, vec![(CoordAndDir::new(start, 0), 0_u32)]);
    queue.push(Reverse(0));

    fn add_to_queue(
        queue: &mut BinaryHeap<Reverse<u32>>,
        cost_map: &mut HashMap<u32, Vec<(CoordAndDir, u32)>>,
        cost: u32,
        pos_and_dir: CoordAndDir,
        prev_dir: u32,
    ) {
        let vec = cost_map.entry(cost).or_default();
        if vec.is_empty() {
            queue.push(Reverse(cost));
        }
        vec.push((pos_and_dir, prev_dir));
    }

    let mut total1 = u32::MAX;

    while let Some(Reverse(cost)) = queue.pop() {
        if cost > total1 {
            break;
        }
        let vec = cost_map.remove(&cost).unwrap();

        for (pos_and_dir, prev_dir) in vec {
            let (pos, dir) = pos_and_dir.into();
            if field[pos] == b'E' {
                total1 = cost;
                let e = &mut reverse[pos][dir as usize];
                *e |= cost | BASE_FLAG << prev_dir;
                continue;
            }

            let e = &mut reverse[pos][dir as usize];
            if *e & FLAG_MASK == 0 {
                *e = cost | BASE_FLAG << prev_dir;
            } else {
                if *e & COST_MASK == cost {
                    *e |= BASE_FLAG << prev_dir;
                }
                continue;
            }

            for (new_dir, add_cost) in [(dir, 1), (dir + 1 & 3, 1001), (dir + 3 & 3, 1001)] {
                let new_pos = pos + dir_coord(new_dir);
                if field[new_pos] == b'#' {
                    continue;
                }

                let new_cost = cost + add_cost;
                add_to_queue(
                    &mut queue,
                    &mut cost_map,
                    new_cost,
                    (new_pos, new_dir).into(),
                    dir,
                );
            }

            if pos == start && field[start.left()] != b'#' {
                let new_pos = start.left();
                add_to_queue(&mut queue, &mut cost_map, 2001, (new_pos, 2).into(), dir);
            }
        }
    }

    if total1 == u32::MAX {
        println!("No path found!");
        return Ok(());
    }

    let mut visited =
        FieldMutView::create_with_value(false, field.width(), field.width(), field.height());

    fn count_reverse_tiles(
        reverse: &mut FieldMutView<[u32; 4]>,
        visited: &mut FieldMutView<bool>,
        mut pos_dir: CoordAndDir,
        end: Coord<i32>,
    ) -> u32 {
        let mut total = 0;
        loop {
            let (pos, dir) = pos_dir.into();
            total += !visited[pos].post_inc() as u32;
            let e = &mut reverse[pos][dir as usize];
            let dirs = *e >> FLAG_SHIFT;
            if pos == end || dirs == 0 {
                return total;
            }
            *e = 0;
            let prev_pos = pos - dir_coord(dir);
            let lowest_dir = dirs.trailing_zeros();
            for new_dir in lowest_dir + 1..4 {
                if dirs & 1 << new_dir == 0 {
                    continue;
                }
                total += count_reverse_tiles(reverse, visited, (prev_pos, new_dir).into(), end);
            }
            pos_dir = (prev_pos, lowest_dir).into();
        }
    }

    let mut total2 = 0;
    for dir in 0..4 {
        total2 += count_reverse_tiles(
            &mut reverse,
            &mut visited,
            CoordAndDir::new(end, dir),
            start,
        );
    }

    //println!("{reverse_path:#?}");

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
