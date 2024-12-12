#![allow(dead_code)]

use anyhow::Result;
use util::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Region {
    area: i32,
    perimeter: i32,
    sides: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum RegionRef {
    Value(Region),
    Ref(usize),
}

impl RegionRef {
    fn as_value(&self) -> Option<&Region> {
        match self {
            RegionRef::Value(region) => Some(region),
            _ => None,
        }
    }

    fn as_value_mut(&mut self) -> Option<&mut Region> {
        match self {
            RegionRef::Value(region) => Some(region),
            _ => None,
        }
    }

    fn as_ref(&self) -> Option<usize> {
        match self {
            RegionRef::Ref(i) => Some(*i),
            _ => None,
        }
    }
}

fn resolve_offset(regions: &Vec<RegionRef>, mut offset: usize) -> usize {
    loop {
        match &regions[offset] {
            RegionRef::Value(_) => break offset,
            &RegionRef::Ref(i) => offset = i,
        }
    }
}

fn main() -> Result<()> {
    let input = open_input("day12")?;
    let field = FieldView::from(&input);
    let mut regions = Vec::with_capacity(1000);
    let mut regids = vec![0; field.data().len()];

    const NEIGHBORS: [Coord<i32>; 4] = [coord(-1, -1), coord(0, -1), coord(1, -1), coord(-1, 0)];

    for coord in field.coords::<i32>() {
        let offset = field.offset(coord.x, coord.y);
        let c = field[offset];
        let mut ids = [0, 0];
        let mut numids = 0;
        let mut mask = 0;

        /*

            mask is a bitpattern that represents the neighbors of the current cell c
            .....
            .124.
            .8c..
            .....

            There are 16 possible situations (a and b denoting regions with the same character but possibly different ids):

            Situation | Value | delta | delta | notes
                      |       | perim | sides |
            .....           0                   n/a
            ..c..

            .a...           1                   n/a
            ..c..

            ..a..           2      +2       0
            ..c..

            .aa..           3      +2      +2
            ..c..

            ...a.           4                   n/a
            ..c..

            .a.a.           5                   n/a
            ..c..

            ..aa.           6      +2      +2
            ..c..

            .aaa.           7      +2      +4
            ..c..

            .....           8      +2       0
            .ac..

            .a...           9      +2      +2
            .ac..

            ..a..          10       0      -2   merge a and b if different ids, delta relative to merged values
            .bc..

            .aa..          11       0      -2
            .ac..

            ...a.          12      +2       0
            .ac..

            .a.a.          13      +2      +2
            .ac..

            ..aa.          14       0       0   merge a and b if different ids, delta relative to merged values
            .bc..

            .aaa.          15       0       0
            .ac..
        */

        const PERIM_DELTA: [i32; 16] = [0, 0, 2, 2, 0, 0, 2, 2, 2, 2, 0, 0, 2, 2, 0, 0];
        const SIDE_DELTA: [i32; 16] = [0, 0, 0, 2, 0, 0, 2, 4, 0, 2, -2, -2, 0, 2, 0, 0];

        for i in 0..4 {
            let n = coord + NEIGHBORS[i];
            if c == *field.get_or(n.x, n.y, &b' ') {
                mask |= 1 << i;
            }
        }

        // print!("{} at {coord} [mask {mask}]: ", to_str(std::slice::from_ref(&c)));

        if mask & 8 == 8 {
            ids[numids.pre_inc()] = resolve_offset(&regions, regids[offset - 1]);
        }

        if mask & 2 == 2 {
            ids[numids.pre_inc()] =
                resolve_offset(&regions, regids[offset - field.stride() as usize]);
        }

        match mask {
            0 | 1 | 4 | 5 => {
                regids[offset] = regions.len();
                regions.push(RegionRef::Value(Region {
                    area: 1,
                    perimeter: 4,
                    sides: 4,
                }));
                // println!("new region");
            }
            10 | 14 if ids[0] != ids[1] => {
                let reg0 = regions[ids[0]].as_value().unwrap().clone();
                let reg1 = regions[ids[1]].as_value_mut().unwrap();
                // print!("merge {reg0:?} + {reg1:?}");
                reg1.area += reg0.area + 1;
                reg1.perimeter += reg0.perimeter + PERIM_DELTA[mask as usize];
                reg1.sides += reg0.sides + SIDE_DELTA[mask as usize];
                // println!(" => {reg1:?}");
                regions[ids[0]] = RegionRef::Ref(ids[1]);
                regids[offset] = ids[1];
            }
            16.. => unreachable!(),
            _ => {
                let reg = regions[ids[0]].as_value_mut().unwrap();
                reg.area += 1;
                reg.perimeter += PERIM_DELTA[mask as usize];
                reg.sides += SIDE_DELTA[mask as usize];
                regids[offset] = ids[0];
                // println!("join1 {reg:?}");
            }
        };
    }

    // println!("{}", regions.len());

    let total1 = regions
        .iter()
        .filter_map(|r| match r {
            RegionRef::Value(region) => Some(region.area as u64 * region.perimeter as u64),
            _ => None,
        })
        .sum::<u64>();
    let total2 = regions
        .iter()
        .filter_map(|r| match r {
            RegionRef::Value(region) => Some(region.area as u64 * region.sides as u64),
            _ => None,
        })
        .sum::<u64>();

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
