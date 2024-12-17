use crate::{Decrement, Increment, Primitive, ToPrimitive};
use std::ops::*;

#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

pub const fn coord<T>(x: T, y: T) -> Coord<T> {
    Coord { x, y }
}

pub trait ToCoord<T> {
    fn to(self) -> Coord<T>;
}

impl<T> Coord<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn to<U: Primitive>(self) -> Coord<U>
    where
        T: ToPrimitive<U> + Copy,
    {
        coord(self.x.to(), self.y.to())
    }

    pub fn tuple(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T: Increment + Decrement> Coord<T> {
    pub fn left(&self) -> Self {
        coord(self.x.get_dec(), self.y)
    }

    pub fn right(&self) -> Self {
        coord(self.x.get_inc(), self.y)
    }

    pub fn up(&self) -> Self {
        coord(self.x, self.y.get_dec())
    }

    pub fn down(&self) -> Self {
        coord(self.x, self.y.get_inc())
    }

    pub fn left_up(&self) -> Self {
        coord(self.x.get_dec(), self.y.get_dec())
    }

    pub fn right_up(&self) -> Self {
        coord(self.x.get_inc(), self.y.get_dec())
    }

    pub fn left_down(&self) -> Self {
        coord(self.x.get_dec(), self.y.get_inc())
    }

    pub fn right_down(&self) -> Self {
        coord(self.x.get_inc(), self.y.get_inc())
    }

    pub fn neighbors4(&self) -> [Self; 4] {
        [self.left(), self.right(), self.up(), self.down()]
    }

    pub fn neighbors8(&self) -> [Self; 8] {
        [
            self.left_up(),
            self.up(),
            self.right_up(),
            self.left(),
            self.right(),
            self.left_down(),
            self.down(),
            self.right_down(),
        ]
    }
}

impl<T: Neg<Output = T>> Coord<T> {
    pub fn turn_left(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn turn_right(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
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

impl<T: MulAssign + Copy> MulAssign<T> for Coord<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Coord<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        coord(self.x / rhs, self.y / rhs)
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Coord<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
impl<T: Rem<Output = T> + Copy> Rem<T> for Coord<T> {
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        coord(self.x % rhs, self.y % rhs)
    }
}

impl<T: RemAssign + Copy> RemAssign<T> for Coord<T> {
    fn rem_assign(&mut self, rhs: T) {
        self.x %= rhs;
        self.y %= rhs;
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

impl<T: std::fmt::Display> std::fmt::Display for Coord<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

macro_rules! impl_rhs_mul {
    ($($t:ty),+) => {
        $(
            impl Mul<Coord<$t>> for $t {
                type Output = Coord<$t>;
                fn mul(self, rhs: Coord<$t>) -> Self::Output {
                    rhs * self
                }
            }
        )+
    };
}

impl_rhs_mul!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
