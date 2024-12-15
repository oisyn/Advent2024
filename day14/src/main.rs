#![allow(dead_code)]

use anyhow::Result;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day14")?;

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

    for i in 0..10000 {
        if (i - 37_i32).rem_euclid(101) == 0 && (i - 87_i32).rem_euclid(103) == 0 {
            println!("{i}");
        }
    }

    for i in 0..1 {
        println!("\nSTEP {i}");
        for r in &mut robots {
            r.0 += r.1 * 7916;
            r.0 = coord(r.0.x.rem_euclid(size.x), r.0.y.rem_euclid(size.y));
        }

        for y in 0..size.y {
            for x in 0..size.x {
                let c = robots.iter().filter(|(p, _)| *p == coord(x, y)).count();

                if c == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }

    let total1 = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];
    let total2 = 0;

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
