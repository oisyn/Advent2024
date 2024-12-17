#![allow(dead_code)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use anyhow::Result;
use tinyvec::ArrayVec;
use util::*;

#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct CoordAndDir(u32);

impl CoordAndDir {
    fn new(pos: Coord<i32>, dir: Coord<i32>) -> Self {
        CoordAndDir(
            (pos.x as u32 & 0x3fff)
                | (pos.y as u32 & 0x3fff) << 14
                | (dir.x as u32 & 0x3) << 28
                | (dir.y as u32 & 0x3) << 30,
        )
    }
}

impl From<(Coord<i32>, Coord<i32>)> for CoordAndDir {
    fn from((pos, dir): (Coord<i32>, Coord<i32>)) -> Self {
        CoordAndDir::new(pos, dir)
    }
}

impl From<CoordAndDir> for (Coord<i32>, Coord<i32>) {
    fn from(c: CoordAndDir) -> Self {
        (
            coord((c.0 as i32) << 18 >> 18, (c.0 as i32) << 4 >> 18),
            coord((c.0 as i32) << 2 >> 30, (c.0 as i32) >> 30),
        )
    }
}

fn main() -> Result<()> {
    let input = open_input("day16")?;
    let field = FieldView::from(&input);

    let start =
        field.coord_from_offset::<i32>(field.data().iter().position(|&c| c == b'S').unwrap());
    let end = field.coord_from_offset::<i32>(field.data().iter().position(|&c| c == b'E').unwrap());

    let mut queue = BinaryHeap::with_capacity(1000);
    let mut visited_vec = vec![false; field.width() * field.height()];
    let mut visited = FieldMutView::new(
        visited_vec.as_mut_slice(),
        field.width(),
        field.width(),
        field.height(),
    );

    // fn calc_min_cost(start: Coord<i32>, end: Coord<i32>, dir: Coord<i32>) -> i32 {
    //     let delta = end - start;
    //     let dist = i32::abs(delta.x) + i32::abs(delta.y);

    //     match (i32::signum(dir.x * delta.x), i32::signum(dir.y * delta.y)) {
    //         (-1, _) | (_, -1) => dist + 2000,
    //         (1, _) if delta.y == 0 => dist,
    //         (_, 1) if delta.x == 0 => dist,
    //         _ => dist + 1000,
    //     }
    // }

    queue.push(Reverse((
        // calc_min_cost(start, end, coord(1, 0)),
        0,
        CoordAndDir::new(start, coord(1, 0)),
        CoordAndDir::new(start, coord(0, 0)),
    )));

    let mut total1 = i32::MAX;
    let mut reverse_path = HashMap::<_, (i32, ArrayVec<[CoordAndDir; 4]>)>::with_capacity(1000);

    while let Some(Reverse((/*_,*/ cost, pos_and_dir, prev_pos_and_dir))) = queue.pop() {
        if cost > total1 {
            break;
        }

        let (pos, dir) = pos_and_dir.into();
        // print!("{pos}-{dir} @ {cost}");
        if field[pos] == b'E' {
            total1 = cost;
            let e = reverse_path
                .entry(CoordAndDir::new(pos, coord(0, 0)))
                .or_default();
            e.0 = cost;
            // println!(" x{}. END!", e.1.len());
            e.1.push(prev_pos_and_dir);
            continue;
        }

        let e = reverse_path.entry(pos_and_dir).or_default();
        // println!(" x{}", e.1.len());
        if e.1.is_empty() {
            e.0 = cost;
            e.1.push(prev_pos_and_dir);
        } else {
            if e.0 == cost {
                e.1.push(prev_pos_and_dir);
            }
            continue;
        }

        for (new_dir, add_cost) in [(dir, 1), (dir.turn_left(), 1001), (dir.turn_right(), 1001)] {
            let new_pos = pos + new_dir;
            if field[new_pos] == b'#' {
                continue;
            }

            let new_cost = cost + add_cost;
            // let new_min_cost = new_cost + calc_min_cost(new_pos, end, new_dir);
            queue.push(Reverse((
                // new_min_cost,
                new_cost,
                CoordAndDir::new(new_pos, new_dir),
                pos_and_dir,
            )));
        }

        if pos == start && field[start.left()] != b'#' {
            let new_pos = start.left();
            let new_cost = cost + 2001;
            // let new_min_cost = new_cost + calc_min_cost(new_pos, end, coord(0, 1));
            queue.push(Reverse((
                // new_min_cost,
                new_cost,
                CoordAndDir::new(new_pos, coord(0, 1)),
                pos_and_dir,
            )));
        }
    }

    if !reverse_path.contains_key(&CoordAndDir::new(end, coord(0, 0))) {
        println!("No path found!");
        return Ok(());
    }

    // let mut field_vec = Vec::from(field.data());
    // let mut field = FieldMutView::new(
    //     field_vec.as_mut_slice(),
    //     field.width(),
    //     field.stride(),
    //     field.height(),
    // );

    fn count_reverse_tiles(
        reverse_path: &HashMap<CoordAndDir, (i32, ArrayVec<[CoordAndDir; 4]>)>,
        visited: &mut FieldMutView<bool>,
        // dbg: &mut FieldMutView<u8>,
        mut prev: CoordAndDir,
        end: Coord<i32>,
    ) -> i32 {
        let mut total = 0;
        loop {
            let (pos, _) = prev.into();
            // dbg[pos] = b'O';
            let e = reverse_path.get(&prev).unwrap();
            total += !visited[pos].post_inc() as i32;
            // println!("{pos} +{total} x{}", e.1.len());
            if pos == end {
                return total;
            }
            for &prev in &e.1[1..] {
                total += count_reverse_tiles(reverse_path, visited, /*dbg,*/ prev, end);
            }
            prev = e.1[0];
        }
    }

    let total2 = count_reverse_tiles(
        &reverse_path,
        &mut visited,
        // &mut field,
        CoordAndDir::new(end, coord(0, 0)),
        start,
    );
    // println!("{}", to_str(field.data()));

    //println!("{reverse_path:#?}");

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
