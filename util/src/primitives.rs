pub trait AnyInt: Copy {
    fn to_isize(self) -> isize;
    fn to_usize(self) -> usize;
    fn from_isize(v: isize) -> Self;
    fn from_usize(v: usize) -> Self;
}

pub trait Primitive: Copy {}
pub trait ToPrimitive<T: Primitive> {
    fn to(self) -> T;
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

macro_rules! impl_uint_anyint {
    ($($t:ty),+) => {
        $(
            impl AnyInt for $t {
                fn to_usize(self) -> usize {
                    self as usize
                }
                fn to_isize(self) -> isize {
                    (self as usize) as isize
                }
                fn from_usize(v: usize) -> Self {
                    v as Self
                }
                fn from_isize(v: isize) -> Self {
                    (v as usize) as Self
                }
            }
        )+
    };
}

macro_rules! impl_sint_anyint {
    ($($t:ty),+) => {
        $(
            impl AnyInt for $t {
                fn to_usize(self) -> usize {
                    (self as isize) as usize
                }
                fn to_isize(self) -> isize {
                    self as isize
                }
                fn from_usize(v: usize) -> Self {
                    (v as isize) as Self
                }
                fn from_isize(v: isize) -> Self {
                    v as Self
                }
            }
        )+
    };
}

impl_uint_anyint!(u8, u16, u32, u64, usize);
impl_sint_anyint!(i8, i16, i32, i64, isize);

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

impl_primitive!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
