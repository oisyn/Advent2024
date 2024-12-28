#![allow(dead_code)]

use std::{collections::HashMap, hash::Hash};

use util::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
enum Op {
    #[default]
    Nop,
    And,
    Or,
    Xor,
}

impl Op {
    fn apply(self, a: u8, b: u8) -> u8 {
        match self {
            Op::Nop => 0,
            Op::And => a & b,
            Op::Or => a | b,
            Op::Xor => a ^ b,
        }
    }
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s.as_bytes()[0] {
            b'A' => Op::And,
            b'O' => Op::Or,
            b'X' => Op::Xor,
            _ => panic!("Invalid op: {}", s),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Annotation {
    #[default]
    None,
    X(u8),
    Y(u8),
    HalfAdd(u8),
    HalfCarry(u8),
    FullAdd(u8),
    PartialFullCarry(u8),
    FullCarry(u8),
}

impl Annotation {
    fn expected_parts(self) -> (Op, Annotation, Annotation) {
        use Annotation::*;
        use Op::*;

        match self {
            HalfAdd(x) => (Xor, X(x), Y(x)),
            HalfCarry(x) => (And, X(x), Y(x)),
            FullAdd(1) => (Xor, HalfAdd(1), HalfCarry(0)),
            FullAdd(x @ 2..) => (Xor, HalfAdd(x), FullCarry(x - 1)),
            PartialFullCarry(1) => (And, HalfAdd(1), HalfCarry(0)),
            PartialFullCarry(x @ 2..) => (And, HalfAdd(x), FullCarry(x - 1)),
            FullCarry(x) => (Or, HalfCarry(x), PartialFullCarry(x)),
            _ => Default::default(),
        }
    }
}

#[aoc_day]
fn solve(input: Input) -> impl AocResult {
    let mut lines = input.lines();
    let mut names = HashMap::with_capacity(500);
    let mut data = vec![0_u8; 64];

    for l in lines.by_ref() {
        if l.is_empty() {
            break;
        }

        let mut p = Parser::new(l);
        let reg = p.take(3);
        let val = p.skip(2).take(1).as_bytes()[0] - b'0';

        names.insert(reg, data.len());
        data.push(val);
    }

    let mut sources = Vec::with_capacity(1000);
    let mut max_z = 0;

    for l in lines {
        let mut p = Parser::new(l);
        let in1 = p.take(3);
        let op = p.skip(1).take_while(|c| c != b' ');
        let in2 = p.skip(1).take(3);
        let out = p.expect(" -> ").take(3);

        let in1 = *names.entry(in1).or_insert_with(|| {
            let n = data.len();
            data.push(0);
            n
        });
        let in2 = *names.entry(in2).or_insert_with(|| {
            let n = data.len();
            data.push(0);
            n
        });

        let out = if out.as_bytes()[0] == b'z' {
            let n = ((out.as_bytes()[1] - b'0') * 10 + out.as_bytes()[2] - b'0') as usize;
            names.insert(out, n);
            max_z = max_z.max(n);
            n
        } else {
            *names.entry(out).or_insert_with(|| {
                let n = data.len();
                data.push(0);
                n
            })
        };
        sources.resize(
            [sources.len(), in1 + 1, in2 + 1, out + 1]
                .into_iter()
                .max()
                .unwrap(),
            Default::default(),
        );
        sources[out] = (in1, in2, Op::from(op));
    }

    fn eval(data: &mut [u8], sources: &[(usize, usize, Op)], visited: &mut [bool], n: usize) -> u8 {
        if visited[n] || sources[n].2 == Op::Nop {
            return data[n];
        }

        visited[n] = true;
        let (in1, in2, op) = sources[n];
        let v1 = eval(data, sources, visited, in1);
        let v2 = eval(data, sources, visited, in2);
        let r = op.apply(v1, v2);
        data[n] = r;
        r
    }

    let mut visited = vec![false; data.len()];
    let mut total1 = 0;
    for i in 0..=max_z {
        total1 |= eval(&mut data, &sources, &mut visited, i) as (u64) << i;
    }

    fn find_gate(
        annotation: Annotation,
        annotations: &mut [Annotation],
        sources: &mut [(usize, usize, Op)],
        names: &HashMap<&str, usize>,
        annotated_gates: &mut HashMap<Annotation, usize>,
        swaps: &mut Vec<(usize, usize)>,
    ) -> usize {
        use Annotation::*;
        if let Some(&g) = annotated_gates.get(&annotation) {
            return g;
        }
        match annotation {
            X(i) => {
                let name = [b'x', b'0' + i / 10, b'0' + i % 10];
                let n = names[to_str(&name)];
                annotations[n] = X(i);
                annotated_gates.insert(X(i), n);
                return n;
            }
            Y(i) => {
                let name = [b'y', b'0' + i / 10, b'0' + i % 10];
                let n = names[to_str(&name)];
                annotations[n] = Y(i);
                annotated_gates.insert(Y(i), n);
                return n;
            }
            _ => {}
        }

        let (op, a1, a2) = annotation.expected_parts();
        let in1 = find_gate(a1, annotations, sources, names, annotated_gates, swaps);
        let in2 = find_gate(a2, annotations, sources, names, annotated_gates, swaps);
        if let Some(idx) = sources
            .iter()
            .position(|&e| e == (in1, in2, op) || e == (in2, in1, op))
        {
            annotations[idx] = annotation;
            annotated_gates.insert(annotation, idx);
            return idx;
        }

        let Some(idx) = sources.iter().position(|&(sin1, sin2, sop)| {
            sop == op && (sin1 == in1 || sin2 == in1 || sin1 == in2 || sin2 == in2)
        }) else {
            unreachable!("No gate found with partial annotation ({a1:?} {op:?} {a2:?})");
        };

        let (sin1, sin2, _) = sources[idx];

        let swap = if sin1 == in1 || sin2 == in1 {
            let other = if sin1 == in1 { sin2 } else { sin1 };
            (other, in2)
        } else {
            let other = if sin1 == in2 { sin2 } else { sin1 };
            (other, in1)
        };

        swaps.push(swap);
        sources.swap(swap.0, swap.1);
        annotated_gates.remove(&annotations[swap.0]);
        annotated_gates.remove(&annotations[swap.1]);
        if annotations[swap.0] != None {
            annotated_gates.insert(annotations[swap.0], swap.1);
        }
        if annotations[swap.1] != None {
            annotated_gates.insert(annotations[swap.1], swap.0);
        }
        annotations.swap(swap.0, swap.1);

        annotations[idx] = annotation;
        annotated_gates.insert(annotation, idx);
        idx
    }

    let mut annotations = vec![Annotation::None; data.len()];
    let mut annotated_gates = HashMap::with_capacity(data.len());
    let mut swaps = Vec::with_capacity(4);
    for i in 0..=max_z {
        let a = match i {
            0 => Annotation::HalfAdd(0),
            i if i == max_z => Annotation::FullCarry(i as u8 - 1),
            i => Annotation::FullAdd(i as u8),
        };
        let idx = find_gate(
            a,
            &mut annotations,
            &mut sources,
            &names,
            &mut annotated_gates,
            &mut swaps,
        );

        if idx != i {
            swaps.push((i, idx));
            sources.swap(i, idx);
            annotated_gates.remove(&annotations[i]);
            annotated_gates.remove(&annotations[idx]);
            if annotations[i] != Annotation::None {
                annotated_gates.insert(annotations[i], idx);
            }
            if annotations[idx] != Annotation::None {
                annotated_gates.insert(annotations[idx], i);
            }
            annotations.swap(i, idx);
        }
    }

    let mut data_names = vec!["--"; data.len()];
    for (&n, &i) in &names {
        data_names[i] = n;
    }

    let mut swapped_names = swaps
        .into_iter()
        .flat_map(|(a, b)| [data_names[a], data_names[b]])
        .collect::<Vec<_>>();
    swapped_names.sort();
    let total2 = swapped_names.join(",");

    (total1, total2)
}
