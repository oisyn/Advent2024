use std::ops::*;

#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

pub const fn coord<T>(x: T, y: T) -> Coord<T> {
    Coord { x, y }
}

impl<T> Coord<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Neg<Output = T>> Neg for Coord<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

impl<T: Add<Output = T>> Add for Coord<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        coord(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: AddAssign> AddAssign for Coord<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Coord<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        coord(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: SubAssign> SubAssign for Coord<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Coord<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        coord(self.x * rhs, self.y * rhs)
    }
}

impl<T: MulAssign> MulAssign for Coord<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Coord<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        coord(self.x / rhs, self.y / rhs)
    }
}

impl<T: DivAssign> DivAssign for Coord<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
impl<T: Rem<Output = T> + Copy> Rem<T> for Coord<T> {
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        coord(self.x % rhs, self.y % rhs)
    }
}

impl<T: RemAssign> RemAssign for Coord<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

impl<T> From<(T, T)> for Coord<T> {
    fn from(value: (T, T)) -> Self {
        coord(value.0, value.1)
    }
}

impl<T> From<Coord<T>> for (T, T) {
    fn from(value: Coord<T>) -> Self {
        (value.x, value.y)
    }
}

macro_rules! impl_rhs_mul {
    ($t:ty) => {
        impl Mul<Coord<$t>> for $t {
            type Output = Coord<$t>;
            fn mul(self, rhs: Coord<$t>) -> Self::Output {
                rhs * self
            }
        }
    };
}

impl_rhs_mul!(u8);
impl_rhs_mul!(u16);
impl_rhs_mul!(u32);
impl_rhs_mul!(u64);
impl_rhs_mul!(usize);

impl_rhs_mul!(i8);
impl_rhs_mul!(i16);
impl_rhs_mul!(i32);
impl_rhs_mul!(i64);
impl_rhs_mul!(isize);

impl_rhs_mul!(f32);
impl_rhs_mul!(f64);
