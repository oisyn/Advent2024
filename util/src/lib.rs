mod parser;
pub use parser::*;

mod input;
pub use input::*;

mod primitives;
pub use primitives::*;

mod field;
pub use field::*;

mod coord;
pub use coord::*;

pub fn to_str(b: &[u8]) -> &str {
    unsafe { std::str::from_utf8_unchecked(b) }
}

pub fn is_nl(c: u8) -> bool {
    c == b'\r' || c == b'\n'
}

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Copy + Default + Eq + std::ops::Rem<T, Output = T>,
{
    let zero = Default::default();
    while b != zero {
        (a, b) = (b, a % b);
    }
    a
}

mod helper {
    pub trait DivEuclid {
        fn div_euclid(self, other: Self) -> Self;
    }

    macro_rules! impl_div_euclid {
        ($($t:ty),+) => {
            $(
                impl DivEuclid for $t {
                    fn div_euclid(self, other: Self) -> Self {
                        self.div_euclid(other)
                    }
                }
            )+
        };
    }

    impl_div_euclid!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);
}

pub fn extended_euclidian<T>(mut a: T, mut b: T) -> (T, T, T)
where
    T: Copy
        + Default
        + Eq
        + std::ops::Sub<T, Output = T>
        + std::ops::Mul<T, Output = T>
        + Increment
        + helper::DivEuclid,
{
    let zero = <T as Default>::default();
    let one = zero.get_inc();
    let (mut os, mut s) = (one, zero);
    let (mut ot, mut t) = (zero, one);

    while b != zero {
        let q = a.div_euclid(b);
        (a, b) = (b, a - q * b);
        (os, s) = (s, os - q * s);
        (ot, t) = (t, ot - q * t);
    }

    (a, os, ot)
}
