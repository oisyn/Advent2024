use crate::{coord, Coord, Input};
use std::{iter::StepBy, ops::Index};

pub trait AnyInt: Copy {
    fn to_isize(self) -> isize;
    fn to_usize(self) -> usize;
    fn from_isize(v: isize) -> Self;
    fn from_usize(v: usize) -> Self;
}

pub struct FieldView<'a, T> {
    data: &'a [T],
    width: usize,
    height: usize,
    stride: usize,
}

impl<'a, T> FieldView<'a, T> {
    pub fn new(data: &'a [T], width: usize, stride: usize, height: usize) -> Self {
        Self {
            data: &data[..height * stride - stride + width],
            width,
            height,
            stride,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn offset<I: AnyInt>(&self, x: I, y: I) -> usize {
        y.to_usize() * self.stride + x.to_usize()
    }

    pub fn tuple_from_offset<I: AnyInt>(&self, o: usize) -> (I, I) {
        (
            AnyInt::from_usize(o % self.stride),
            AnyInt::from_usize(o / self.stride),
        )
    }

    pub fn coord_from_offset<I: AnyInt>(&self, o: usize) -> Coord<I> {
        self.tuple_from_offset(o).into()
    }

    pub fn data(&self) -> &[T] {
        self.data
    }

    pub fn get<I: AnyInt>(&self, x: I, y: I) -> &T {
        &self.data[self.offset(x, y)]
    }

    pub fn get_or<'r, I: AnyInt>(&'r self, x: I, y: I, alt: &'r T) -> &'r T {
        if (0..self.width as isize).contains(&(x.to_isize()))
            && (0..self.height as isize).contains(&(y.to_isize()))
        {
            &self.data[self.offset(x, y)]
        } else {
            alt
        }
    }

    pub fn get_by_offset_or<'r>(&'r self, off: usize, alt: &'r T) -> &'r T {
        let pos = self.tuple_from_offset::<usize>(off);
        self.get_or(pos.0, pos.1, alt)
    }

    pub fn row(&self, index: usize) -> &[T] {
        let o = index * self.stride;
        &self.data[o..o + self.width]
    }

    pub fn col(&self, index: usize) -> FieldColumn<'a, T> {
        FieldColumn {
            data: &self.data[index..],
            stride: self.stride,
            height: self.height,
        }
    }

    pub fn rows(&self) -> FieldRows<'a, T> {
        FieldRows(self.clone())
    }

    pub fn cols(&self) -> FieldCols<'a, T> {
        FieldCols(self.clone())
    }

    pub fn offsets(&self) -> impl Iterator<Item = usize> {
        let (w, h, s) = (self.width, self.height, self.stride);
        (0..h).flat_map(move |y| (0..w).map(move |x| y * s + x))
    }

    pub fn coords<I: AnyInt>(&self) -> impl Iterator<Item = Coord<I>> {
        let (w, h) = (self.width, self.height);
        (0..h).flat_map(move |y| (0..w).map(move |x| coord(I::from_usize(x), I::from_usize(y))))
    }
}

impl<'a, T> Clone for FieldView<'a, T> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<'a, T, I: AnyInt> Index<I> for FieldView<'a, T> {
    type Output = T;
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index.to_usize()]
    }
}

impl<'a, T, I: AnyInt> Index<(I, I)> for FieldView<'a, T> {
    type Output = T;
    fn index(&self, pos: (I, I)) -> &Self::Output {
        &self.data[self.offset(pos.0, pos.1)]
    }
}

impl<'a, T, I: AnyInt> Index<Coord<I>> for FieldView<'a, T> {
    type Output = T;
    fn index(&self, pos: Coord<I>) -> &Self::Output {
        &self.data[self.offset(pos.x, pos.y)]
    }
}

impl<'a> From<&'a Input> for FieldView<'a, u8> {
    fn from(input: &'a Input) -> Self {
        let b = input.bytes();
        let width = b.iter().position(|&c| c == b'\r' || c == b'\n').unwrap();
        let stride = width + 1 + ((b[width] == b'\r') as usize);
        let height = b.len().div_ceil(stride);
        Self::new(b, width, stride, height)
    }
}

pub struct BorderedFieldView<'a, T> {
    view: FieldView<'a, T>,
    border: T,
}

impl<'a, T> BorderedFieldView<'a, T> {
    pub fn new(view: FieldView<'a, T>, border: T) -> Self {
        Self { view, border }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        self.view.get_or(x, y, &self.border)
    }

    pub fn width(&self) -> usize {
        self.view.width
    }

    pub fn height(&self) -> usize {
        self.view.height
    }

    pub fn stride(&self) -> usize {
        self.view.stride
    }

    pub fn offset<I: AnyInt>(&self, x: I, y: I) -> usize {
        self.view.offset(x, y)
    }

    pub fn tuple_from_offset<I: AnyInt>(&self, o: usize) -> (I, I) {
        self.view.tuple_from_offset(o)
    }

    pub fn coord_from_offset<I: AnyInt>(&self, o: usize) -> Coord<I> {
        self.view.coord_from_offset(o)
    }

    pub fn data(&self) -> &[T] {
        self.view.data
    }

    pub fn row(&self, index: usize) -> &[T] {
        self.view.row(index)
    }

    pub fn col(&self, index: usize) -> FieldColumn<'a, T> {
        self.view.col(index)
    }

    pub fn rows(&self) -> FieldRows<'a, T> {
        self.view.rows()
    }

    pub fn cols(&self) -> FieldCols<'a, T> {
        self.view.cols()
    }

    pub fn offsets(&self) -> impl Iterator<Item = usize> {
        let (w, h, s) = (self.view.width, self.view.height, self.view.stride);
        (0..h).flat_map(move |y| (0..w).map(move |x| y * s + x))
    }

    pub fn coords<I: AnyInt>(&self) -> impl Iterator<Item = Coord<I>> {
        let (w, h) = (self.view.width, self.view.height);
        (0..h).flat_map(move |y| (0..w).map(move |x| coord(I::from_usize(x), I::from_usize(y))))
    }
}

impl<'a, T, I: AnyInt> Index<I> for BorderedFieldView<'a, T> {
    type Output = T;
    fn index(&self, index: I) -> &Self::Output {
        self.view.get_by_offset_or(index.to_usize(), &self.border)
    }
}

impl<'a, T, I: AnyInt> Index<(I, I)> for BorderedFieldView<'a, T> {
    type Output = T;
    fn index(&self, pos: (I, I)) -> &Self::Output {
        self.view.get_or(pos.0, pos.1, &self.border)
    }
}

impl<'a, T, I: AnyInt> Index<Coord<I>> for BorderedFieldView<'a, T> {
    type Output = T;
    fn index(&self, pos: Coord<I>) -> &Self::Output {
        self.view.get_or(pos.x, pos.y, &self.border)
    }
}

#[derive(Clone)]
pub struct FieldRows<'a, T>(FieldView<'a, T>);

impl<'a, T> ExactSizeIterator for FieldRows<'a, T> {
    fn len(&self) -> usize {
        self.0.height
    }
}

impl<'a, T> Iterator for FieldRows<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.height == 0 {
            None
        } else {
            self.0.height -= 1;
            let d = &self.0.data[0..self.0.width];
            self.0.data = &self.0.data[self.0.stride.min(self.0.data.len())..];
            Some(d)
        }
    }
}

impl<'a, T> DoubleEndedIterator for FieldRows<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.0.height == 0 {
            None
        } else {
            self.0.height -= 1;
            let d = &self.0.data[self.0.data.len() - self.0.width..];
            self.0.data = &self.0.data[..self.0.data.len().max(self.0.stride) - self.0.stride];
            Some(d)
        }
    }
}

#[derive(Clone)]
pub struct FieldCols<'a, T>(FieldView<'a, T>);

impl<'a, T> ExactSizeIterator for FieldCols<'a, T> {
    fn len(&self) -> usize {
        self.0.width
    }
}

impl<'a, T> Iterator for FieldCols<'a, T> {
    type Item = FieldColumn<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.width == 0 {
            None
        } else {
            self.0.width -= 1;
            let d = self.0.data;
            self.0.data = &self.0.data[1..];
            Some(FieldColumn {
                data: d,
                stride: self.0.stride,
                height: self.0.height,
            })
        }
    }
}

impl<'a, T> DoubleEndedIterator for FieldCols<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.0.width == 0 {
            None
        } else {
            self.0.width -= 1;
            let d = &self.0.data[self.0.width..];
            Some(FieldColumn {
                data: d,
                stride: self.0.stride,
                height: self.0.height,
            })
        }
    }
}

pub struct FieldColumn<'a, T> {
    data: &'a [T],
    stride: usize,
    height: usize,
}

impl<'a, T> FieldColumn<'a, T> {
    pub fn len(&self) -> usize {
        self.height
    }
}

impl<'a, T> Clone for FieldColumn<'a, T> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<'a, T> std::ops::Index<usize> for FieldColumn<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.stride]
    }
}

impl<'a, T> IntoIterator for FieldColumn<'a, T> {
    type Item = &'a T;
    type IntoIter = StepBy<std::slice::Iter<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter().step_by(self.stride)
    }
}

impl<'a, T: PartialEq> PartialEq for FieldColumn<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.clone().into_iter().eq(other.clone().into_iter())
    }
}

impl<'a, T: Eq> Eq for FieldColumn<'a, T> {}

impl<'a, T: PartialOrd> PartialOrd for FieldColumn<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.clone()
            .into_iter()
            .partial_cmp(other.clone().into_iter())
    }
}

impl<'a, T: Ord> Ord for FieldColumn<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.clone().into_iter().cmp(other.clone().into_iter())
    }
}

impl<'a, T: std::fmt::Debug> std::fmt::Debug for FieldColumn<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone().into_iter()).finish()
    }
}

macro_rules! impl_uint_anyint {
    ($t:ty) => {
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
    };
}

macro_rules! impl_sint_anyint {
    ($t:ty) => {
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
    };
}

impl_uint_anyint!(u8);
impl_uint_anyint!(u16);
impl_uint_anyint!(u32);
impl_uint_anyint!(u64);
impl_uint_anyint!(usize);

impl_sint_anyint!(i8);
impl_sint_anyint!(i16);
impl_sint_anyint!(i32);
impl_sint_anyint!(i64);
impl_sint_anyint!(isize);
