use crate::*;

pub struct Parser<'a> {
    buf: &'a [u8],
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { buf: s.as_bytes() }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn at_end(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn skip(&mut self, num: usize) -> &mut Self {
        self.buf = &self.buf[num.min(self.buf.len())..];
        self
    }

    pub fn expect(&mut self, s: &str) -> &mut Self {
        #[cfg(feature = "validation")]
        {
            let c = self.take(s.len());
            if c != s {
                panic!("Validation failed! Parser expected `{s}`, got `{c}`");
            }
        }

        #[cfg(not(feature = "validation"))]
        self.skip(s.len());

        self
    }

    pub fn peek_char(&self) -> Option<u8> {
        if self.buf.is_empty() {
            None
        } else {
            Some(self.buf[0])
        }
    }

    pub fn take_char(&mut self) -> Option<u8> {
        if self.buf.is_empty() {
            None
        } else {
            let r = Some(self.buf[0]);
            self.buf = &self.buf[1..];
            r
        }
    }

    pub fn peek(&mut self, len: usize) -> &'a str {
        let len = len.min(self.buf.len());
        to_str(&self.buf[..len])
    }

    pub fn take(&mut self, len: usize) -> &'a str {
        let len = len.min(self.buf.len());
        let str = to_str(&self.buf[..len]);
        self.buf = &self.buf[len..];
        str
    }

    pub fn take_while(&mut self, mut f: impl FnMut(u8) -> bool) -> &'a str {
        let len = 'len: {
            for i in 0..self.buf.len() {
                if !f(self.buf[i]) {
                    break 'len i;
                }
            }
            self.buf.len()
        };
        self.take(len)
    }

    pub fn peek_remainder(&mut self) -> &'a str {
        to_str(&self.buf)
    }

    pub fn remainder(&mut self) -> &'a str {
        let r = to_str(self.buf);
        self.buf = &self.buf[self.buf.len()..];
        r
    }

    pub fn consume<T>(&mut self, f: impl FnOnce(&[u8]) -> (usize, T)) -> T {
        let (len, r) = f(self.buf);
        self.skip(len);
        r
    }

    pub fn parse<T: FromParser<'a>>(&mut self) -> Option<T> {
        <T as FromParser<'a>>::parse_from(self)
    }

    pub fn parse_with<T: 'a>(&mut self, f: impl FnOnce(&mut Self) -> Option<T>) -> Option<T> {
        f(self)
    }

    pub fn parse_iter<'p, 's, T: FromParser<'a>>(
        &'p mut self,
        separator: &'s str,
    ) -> ParseIter<'a, 'p, 's, T, impl FnMut(&mut Self) -> Option<T>> {
        self.parse_iter_with(separator, FromParser::parse_from)
    }

    pub fn parse_iter_with<'p, 's, T: 'a, F: FnMut(&mut Self) -> Option<T>>(
        &'p mut self,
        separator: &'s str,
        f: F,
    ) -> ParseIter<'a, 'p, 's, T, F> {
        ParseIter {
            parser: self,
            func: f,
            sep: separator,
            skip: false,
        }
    }
}

pub trait FromParser<'p>: Sized + 'p {
    fn parse_from(parser: &mut Parser<'p>) -> Option<Self>;
}

pub struct ParseIter<'a, 'p, 's, T: 'a, F: FnMut(&mut Parser<'a>) -> Option<T>> {
    parser: &'p mut Parser<'a>,
    func: F,
    sep: &'s str,
    skip: bool,
}

impl<'a, 'p, 's, T, F> Iterator for ParseIter<'a, 'p, 's, T, F>
where
    T: 'a,
    F: FnMut(&mut Parser<'a>) -> Option<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.parser.at_end() {
            return None;
        }

        if std::mem::replace(&mut self.skip, true) {
            self.parser.expect(self.sep);
        }

        (self.func)(self.parser)
    }
}

macro_rules! impl_uint_parser {
    ($t:ty) => {
        impl FromParser<'_> for $t {
            fn parse_from(parser: &mut Parser) -> Option<Self> {
                parser.take_while(|c| c.is_ascii_digit()).parse().ok()
            }
        }
    };
}

macro_rules! impl_sint_parser {
    ($t:ty) => {
        impl FromParser<'_> for $t {
            fn parse_from(parser: &mut Parser) -> Option<Self> {
                let neg = parser.peek_char()? == b'-';
                if neg {
                    parser.skip(1);
                }
                let n = parser
                    .take_while(|c| c.is_ascii_digit())
                    .parse::<$t>()
                    .ok()?;
                if neg {
                    Some(-n)
                } else {
                    Some(n)
                }
            }
        }
    };
}

impl_uint_parser!(u8);
impl_uint_parser!(u16);
impl_uint_parser!(u32);
impl_uint_parser!(u64);
impl_uint_parser!(usize);

impl_sint_parser!(i8);
impl_sint_parser!(i16);
impl_sint_parser!(i32);
impl_sint_parser!(i64);
impl_sint_parser!(isize);
