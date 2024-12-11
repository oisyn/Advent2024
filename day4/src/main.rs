use anyhow::Result;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day4")?;
    let field = FieldView::from(&input);
    let width = field.width();
    let height = field.height();
    let stride = field.stride();
    let data = field.data();
    let istride = stride as isize;

    let mut total1 = 0;
    let mut total2 = 0;

    fn check1(data: &[u8], base: usize, delta: isize) -> i32 {
        let base = base as isize;
        (data[(base + delta) as usize] == b'M'
            && data[(base + 2 * delta) as usize] == b'A'
            && data[(base + 3 * delta) as usize] == b'S') as i32
    }

    let mut rest = data;
    let mut base = 0;
    while !rest.is_empty() {
        let Some(idx) = rest.iter().position(|&c| c == b'X') else {
            break;
        };
        base += idx;

        let (x, y) = field.tuple_from_offset::<usize>(base);
        if x >= 3 {
            total1 += check1(data, base, -1);
            if y >= 3 {
                total1 += check1(data, base, -istride - 1);
            }
            if y <= height - 4 {
                total1 += check1(data, base, istride - 1);
            }
        }
        if x <= width - 4 {
            total1 += check1(data, base, 1);
            if y >= 3 {
                total1 += check1(data, base, -istride + 1);
            }
            if y <= height - 4 {
                total1 += check1(data, base, istride + 1);
            }
        }
        if y >= 3 {
            total1 += check1(data, base, -istride);
        }
        if y <= height - 4 {
            total1 += check1(data, base, istride);
        }

        rest = &rest[idx + 1..];
        base += 1;
    }

    fn check2(data: &[u8], base: usize, stride: usize) -> i32 {
        let pattern = &[
            data[base - stride - 1],
            data[base - stride + 1],
            data[base + stride - 1],
            data[base + stride + 1],
        ];
        (pattern == b"MMSS" || pattern == b"MSMS" || pattern == b"SSMM" || pattern == b"SMSM")
            as i32
    }

    let mut base = stride + 1;
    let mut rest = &data[base..data.len() - stride - 1];
    while !rest.is_empty() {
        let Some(idx) = rest.iter().position(|&c| c == b'A') else {
            break;
        };
        base += idx;
        total2 += check2(data, base, stride);
        rest = &rest[idx + 1..];
        base += 1;
    }

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
