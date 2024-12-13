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

pub trait IncrementalIdentity {
    type Identity;
    fn incremental_identity() -> Self::Identity;
}

pub trait Increment: IncrementalIdentity + Copy + std::ops::AddAssign<Self::Identity> {
    fn pre_inc(&mut self) -> Self {
        let r = *self;
        *self += Self::incremental_identity();
        r
    }

    fn post_inc(&mut self) -> Self {
        *self += Self::incremental_identity();
        *self
    }

    fn get_inc(&self) -> Self {
        self.clone().pre_inc()
    }
}

impl<T> Increment for T where T: IncrementalIdentity + Copy + std::ops::AddAssign<T::Identity> {}

pub trait Decrement: IncrementalIdentity + Copy + std::ops::SubAssign<Self::Identity> {
    fn pre_dec(&mut self) -> Self {
        let r = *self;
        *self -= Self::incremental_identity();
        r
    }

    fn post_dec(&mut self) -> Self {
        *self -= Self::incremental_identity();
        *self
    }

    fn get_dec(&self) -> Self {
        self.clone().pre_dec()
    }
}

impl<T> Decrement for T where T: IncrementalIdentity + Copy + std::ops::SubAssign<T::Identity> {}

macro_rules! impl_additive_identities {
    ($($t:ty),+) => {
        $(
            impl IncrementalIdentity for $t {
                type Identity = $t;
                fn incremental_identity() -> Self::Identity {
                    1 as $t
                }
            }
        )+
    };
}

impl_additive_identities!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

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
