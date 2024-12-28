#![allow(dead_code)]

use std::collections::HashMap;

use util::*;

#[derive(Clone, Debug)]
struct Node<'a> {
    name: &'a str,
    edges: Vec<usize>,
}

#[aoc_day]
fn solve(input: Input) -> impl AocResult {
    let mut nodes = Vec::with_capacity(1000);
    let mut names = HashMap::with_capacity(1000);

    for l in input.lines() {
        let mut p = Parser::new(l);
        let n1 = p.take(2);
        let n2 = p.skip(1).take(2);

        let i1 = *names.entry(n1).or_insert_with(|| {
            let i = nodes.len();
            nodes.push(Node {
                name: n1,
                edges: Vec::new(),
            });
            i
        });

        let i2 = *names.entry(n2).or_insert_with(|| {
            let i = nodes.len();
            nodes.push(Node {
                name: n2,
                edges: Vec::new(),
            });
            i
        });

        nodes[i1].edges.push(i2);
        nodes[i2].edges.push(i1);
    }

    let mut total1 = 0;
    for i0 in 0..nodes.len() {
        for &i1 in nodes[i0].edges.iter().filter(|&&i| i > i0) {
            for &i2 in nodes[i1].edges.iter().filter(|&&i| i > i1) {
                if nodes[i0].edges.contains(&i2)
                    && [i0, i1, i2]
                        .iter()
                        .any(|&i| nodes[i].name.as_bytes()[0] == b't')
                {
                    total1 += 1;
                }
            }
        }
    }

    fn bron_kerbosch(
        r: &mut Vec<usize>,
        mut p: Vec<usize>,
        mut x: Vec<usize>,
        nodes: &Vec<Node>,
    ) -> Vec<usize> {
        if p.is_empty() && x.is_empty() {
            return r.clone();
        }

        let mut ret = Vec::new();

        while let Some(v) = p.pop() {
            r.push(v);
            {
                let mut p = p.clone();
                let mut x = x.clone();
                p.retain(|e| nodes[v].edges.contains(e));
                x.retain(|e| nodes[v].edges.contains(e));
                let new = bron_kerbosch(r, p, x, nodes);
                if ret.len() < new.len() {
                    ret = new;
                }
            }
            r.pop();
            x.push(v);
        }

        ret
    }

    let max = bron_kerbosch(
        &mut Vec::with_capacity(1000),
        (0..nodes.len()).collect(),
        Vec::new(),
        &nodes,
    );

    let mut max_names = max.iter().map(|&i| nodes[i].name).collect::<Vec<_>>();
    max_names.sort();

    (total1, max_names.join(","))
}
