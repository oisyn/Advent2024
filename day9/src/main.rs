use anyhow::Result;
use util::*;

fn sum(from: u64, to: u64) -> u64 {
    (to * (to + 1)).wrapping_sub(from * (from.wrapping_sub(1))) / 2
}

fn main() -> Result<()> {
    let input = open_input("day9")?;
    let data = input.bytes();

    let mut total1 = 0;

    let mut front = 0;
    let mut back = data.len() - 1;
    let mut write = 0;
    let mut last_len = (data[back] - b'0') as u64;

    while front < back {
        let f = (data[front] - b'0') as u64;
        // println!("{f} * {}", front / 2);
        total1 += sum(write, write + f - 1) * (front as u64 / 2);
        write += f;

        front += 1;
        let mut gap = (data[front] - b'0') as u64;
        while gap > 0 {
            let min = last_len.min(gap);
            // println!("{min} * {}", back / 2);
            total1 += sum(write, write + min - 1) * (back as u64 / 2);
            last_len -= min;
            gap -= min;
            write += min;

            if last_len == 0 {
                back -= 2;
                last_len = (data[back] - b'0') as u64;
            }
        }

        front += 1;
    }

    if last_len > 0 {
        // println!("{last_len} * {}", back / 2);
        total1 += sum(write, write + last_len - 1) * (back as u64 / 2);
    }

    let mut gaps = Vec::with_capacity(data.len());
    let mut orig_pos = vec![0; (data.len() + 1) / 2];
    let mut offset = 0;

    for (idx, e) in data.chunks(2).enumerate() {
        let l = (e[0] - b'0') as u64;
        orig_pos[idx as usize] = offset;
        offset += l;
        if e.len() > 1 {
            let g = (e[1] - b'0') as u64;
            gaps.push((offset, g));
            offset += g;
        }
    }

    let mut total2 = 0;

    'outer: for (idx, &len) in data.iter().step_by(2).enumerate().rev() {
        let len = (len - b'0') as u64;
        for gap in &mut gaps {
            if gap.1 < len {
                continue;
            };
            if gap.0 > orig_pos[idx] {
                break;
            }

            let pos = gap.0;
            total2 += sum(pos, pos + len - 1) * idx as u64;

            gap.0 = pos + len;
            gap.1 -= len;

            continue 'outer;
        }

        total2 += sum(orig_pos[idx], orig_pos[idx] + len - 1) * idx as u64;
    }

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
