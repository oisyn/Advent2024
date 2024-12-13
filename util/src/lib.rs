mod parser;
pub use parser::*;

mod input;
pub use input::*;

mod primitives;
pub use primitives::*;

mod fieldview;
pub use fieldview::*;

mod coord;
pub use coord::*;

pub fn to_str(b: &[u8]) -> &str {
    unsafe { std::str::from_utf8_unchecked(b) }
}

pub fn is_nl(c: u8) -> bool {
    c == b'\r' || c == b'\n'
}

pub fn gcd<T>(mut n: T, mut m: T) -> T
where
    T: Copy + Default + Eq + Ord + std::ops::Rem<T, Output = T>,
{
    let zero = Default::default();
    while m != zero {
        (n, m) = (m, n % m);
    }
    n
}
