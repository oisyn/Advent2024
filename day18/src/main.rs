#![allow(dead_code)]

use std::collections::VecDeque;

use anyhow::Result;
use itertools::*;
use util::*;

const EXAMPLE_SIZE: Coord<i32> = coord(7, 7);
const INPUT_SIZE: Coord<i32> = coord(71, 71);

fn main() -> Result<()> {
    let input = open_input("day18")?;
    let coords = input
        .lines()
        .map(|l| {
            Coord::from(
                Parser::new(l)
                    .parse_iter(",")
                    .collect_tuple::<(i32, i32)>()
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let (size, part1_steps) = match input.is_example() {
        true => (EXAMPLE_SIZE, 12),
        false => (INPUT_SIZE, 1024),
    };

    let end_pos = size - coord(1, 1);

    let mut field = FieldMutView::create_with_value(
        u16::MAX,
        size.x as usize,
        size.x as usize,
        size.y as usize,
    );
    let mut visited =
        FieldMutView::create_with_value(false, size.x as usize, size.x as usize, size.y as usize);

    for (step, &c) in coords.iter().enumerate() {
        field[c] = step as u16 + 1;
    }

    let mut queue = VecDeque::with_capacity(1000);
    queue.push_back((coord(0, 0), 0));
    visited[0] = true;

    let total1 = 'done: {
        while let Some((pos, cost)) = queue.pop_front() {
            for n in pos.neighbors4() {
                if n == end_pos {
                    break 'done cost + 1;
                }
                if field.in_bounds_coord(n) && field[n] > part1_steps as u16 && !visited[n] {
                    visited[n] = true;
                    queue.push_back((n, cost + 1));
                }
            }
        }
        panic!("Path not found!");
    };

    let total2 = 'done: {
        let mut queue = VecDeque::with_capacity(1000);
        let mut min = part1_steps + 1;
        let mut max = coords.len() + 1;
        'next: loop {
            if (max - min) == 1 {
                break 'done coords[min];
            }

            queue.clear();
            visited.data_mut().fill(false);
            queue.push_back(coord(0, 0));
            visited[0] = true;
            let mid = (max + min) / 2;
            while let Some(pos) = queue.pop_front() {
                for n in pos.neighbors4() {
                    if n == end_pos {
                        min = mid;
                        continue 'next;
                    }
                    if field.in_bounds_coord(n) && field[n] > mid as u16 && !visited[n] {
                        visited[n] = true;
                        queue.push_back(n);
                    }
                }
            }

            max = mid;
        }
    };

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
