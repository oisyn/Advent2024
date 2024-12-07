use anyhow::Result;
use std::collections::HashMap;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day5")?;

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut lines = input.lines();

    for l in lines.by_ref() {
        if l.is_empty() {
            break;
        }

        let mut p = Parser::new(l);
        let page1 = p.parse().unwrap();
        let page2 = p.expect("|").parse().unwrap();

        rules.entry(page1).or_default().push(page2);
    }

    let mut total1 = 0;
    let mut total2 = 0;

    let mut pages = Vec::with_capacity(1000);
    let mut page_indices = HashMap::with_capacity(1000);

    'nextline: for l in lines {
        pages.clear();
        pages.extend(Parser::new(l).parse_iter::<i32>(","));

        page_indices.clear();

        for (idx, &p) in pages.iter().enumerate() {
            page_indices.insert(p, idx);
        }

        'part1: {
            for (idx, &p) in pages.iter().enumerate() {
                let Some(r) = rules.get(&p) else {
                    continue;
                };

                for &p2 in r {
                    if let Some(&idx2) = page_indices.get(&p2) {
                        if idx2 < idx {
                            break 'part1;
                        }
                    }
                }
            }

            total1 += pages[pages.len() / 2];
            continue 'nextline;
        }

        let mid = pages.len() / 2;
        total2 += *pages
            .select_nth_unstable_by(mid, |&a, &b| {
                use std::cmp::Ordering::*;
                if let Some(r) = rules.get(&a) {
                    if r.contains(&b) {
                        return Less;
                    }
                }
                if let Some(r) = rules.get(&b) {
                    if r.contains(&a) {
                        return Greater;
                    }
                }
                Equal
            })
            .1;
    }

    drop(input);
    println!("{total1} - {total2}");

    Ok(())
}
