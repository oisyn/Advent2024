use util::*;

enum Instruction {
    Mul(i32),
    Do,
    Dont,
}

impl FromParser<'_> for Instruction {
    fn parse_from(parser: &mut Parser<'_>) -> Option<Self> {
        while !parser.at_end() {
            parser.take_while(|c| c != b'm' && c != b'd');
            match parser.peek_remainder() {
                x if x.starts_with("mul(") => {
                    parser.skip(4);
                    let a = parser.parse::<i32>();
                    if a.is_none() || parser.take_char() != Some(b',') {
                        continue;
                    }
                    let b = parser.parse::<i32>();
                    if b.is_none() || parser.take_char() != Some(b')') {
                        continue;
                    }
                    return Some(Self::Mul(a.unwrap() * b.unwrap()));
                }
                x if x.starts_with("do()") => {
                    parser.skip(4);
                    return Some(Self::Do);
                }
                x if x.starts_with("don't()") => {
                    parser.skip(7);
                    return Some(Self::Dont);
                }
                _ => {
                    parser.skip(1);
                }
            }
        }
        None
    }
}

#[aoc_day]
fn solve(input: Input) -> impl AocResult {
    let mut total1 = 0;
    let mut total2 = 0;
    let mut enabled = true;

    for l in input.lines() {
        for i in Parser::new(l).parse_iter::<Instruction>("") {
            match i {
                Instruction::Mul(r) => {
                    total1 += r;
                    if enabled {
                        total2 += r;
                    }
                }
                Instruction::Do => {
                    enabled = true;
                }
                Instruction::Dont => {
                    enabled = false;
                }
            }
        }
    }

    (total1, total2)
}
