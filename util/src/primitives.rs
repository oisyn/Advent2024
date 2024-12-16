pub trait Primitive: Copy + ToPrimitive<usize> + FromPrimitive<usize> {}
pub trait PrimitiveInt: Primitive {}
pub trait PrimitiveFloat: Primitive {}

pub trait ToPrimitive<T: Primitive> {
    fn to(self) -> T;
}

pub trait FromPrimitive<T: Primitive> {
    fn from(value: T) -> Self;
}

impl<T: Primitive, U: ToPrimitive<T> + Primitive> FromPrimitive<U> for T {
    fn from(value: U) -> Self {
        value.to()
    }
}

pub trait Increment: Copy {
    fn pre_inc(&mut self) -> Self;
    fn post_inc(&mut self) -> Self;
    fn get_inc(self) -> Self;
}

pub trait Decrement: Copy {
    fn pre_dec(&mut self) -> Self;
    fn post_dec(&mut self) -> Self;
    fn get_dec(self) -> Self;
}

macro_rules! impl_increments {
    ($($t:ty),+) => {
        $(
            impl Increment for $t {
                fn pre_inc(&mut self) -> Self {
                    *self += 1 as $t;
                    *self
                }
                fn post_inc(&mut self) -> Self {
                    let old = *self;
                    *self += 1 as $t;
                    old
                }
                fn get_inc(self) -> Self {
                    self + 1 as $t
                }
            }

            impl Decrement for $t {
                fn pre_dec(&mut self) -> Self {
                    *self -= 1 as $t;
                    *self
                }
                fn post_dec(&mut self) -> Self {
                    let old = *self;
                    *self -= 1 as $t;
                    old
                }
                fn get_dec(self) -> Self {
                    self - 1 as $t
                }
            }
        )+
    };
}

impl_increments!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

impl Increment for bool {
    fn pre_inc(&mut self) -> Self {
        *self = true;
        true
    }
    fn post_inc(&mut self) -> Self {
        std::mem::replace(self, true)
    }
    fn get_inc(self) -> Self {
        true
    }
}

impl Decrement for bool {
    fn pre_dec(&mut self) -> Self {
        *self = false;
        false
    }
    fn post_dec(&mut self) -> Self {
        std::mem::replace(self, false)
    }
    fn get_dec(self) -> Self {
        false
    }
}

macro_rules! impl_to_primitive {
    ($t:ty => $($u:ty),+) => {
        $(
            impl ToPrimitive<$u> for $t {
                fn to(self) -> $u {
                    self as $u
                }
            }
        )+
    };
}
macro_rules! impl_primitive {
    ($($t:ty),+) => {
        $(
            impl Primitive for $t {}
            impl_to_primitive!($t => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
        )+
    };
}

macro_rules! impl_primitive_int {
    ($($t:ty),+) => {
        $(
            impl PrimitiveInt for $t {}
        )+
    };
}

macro_rules! impl_primitive_float {
    ($($t:ty),+) => {
        $(
            impl PrimitiveFloat for $t {}
        )+
    };
}

impl_primitive!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
impl_primitive_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_primitive_float!(f32, f64);
