use crate::{coord, Coord, FromPrimitive, Input, PrimitiveInt, ToPrimitive};
use std::{
    iter::StepBy,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

pub trait Field {
    type Item;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn stride(&self) -> usize;
    fn data(&self) -> &[Self::Item];

    fn offset<I: PrimitiveInt + ToPrimitive<usize>>(&self, x: I, y: I) -> usize {
        y.to() as usize * self.stride() + x.to()
    }

    fn tuple_from_offset<I: FromPrimitive<usize>>(&self, o: usize) -> (I, I) {
        (
            FromPrimitive::from(o % self.stride()),
            FromPrimitive::from(o / self.stride()),
        )
    }

    fn coord_from_offset<I: FromPrimitive<usize>>(&self, o: usize) -> Coord<I> {
        self.tuple_from_offset(o).into()
    }

    fn in_bounds<I: PrimitiveInt + ToPrimitive<usize>>(&self, x: I, y: I) -> bool {
        x.to() < self.width() && y.to() < self.height()
    }

    fn in_bounds_tuple<I: PrimitiveInt + ToPrimitive<usize>>(&self, p: (I, I)) -> bool {
        self.in_bounds(p.0, p.1)
    }

    fn in_bounds_coord<I: PrimitiveInt + ToPrimitive<usize>>(&self, p: Coord<I>) -> bool {
        self.in_bounds(p.x, p.y)
    }

    fn get<I: PrimitiveInt + ToPrimitive<usize>>(&self, x: I, y: I) -> &Self::Item {
        &self.data()[self.offset(x, y)]
    }

    fn get_or<'r, I: PrimitiveInt + ToPrimitive<usize>>(
        &'r self,
        x: I,
        y: I,
        alt: &'r Self::Item,
    ) -> &'r Self::Item {
        if x.to() < self.width() && y.to() < self.height() {
            &self.data()[self.offset(x, y)]
        } else {
            alt
        }
    }

    fn get_by_offset_or<'r>(&'r self, off: usize, alt: &'r Self::Item) -> &'r Self::Item {
        let pos = self.tuple_from_offset::<usize>(off);
        self.get_or(pos.0, pos.1, alt)
    }

    fn row(&self, index: usize) -> &[Self::Item] {
        let o = index * self.stride();
        &self.data()[o..o + self.width()]
    }

    fn col(&self, index: usize) -> FieldColumn<Self::Item> {
        FieldColumn {
            data: &self.data()[index..],
            stride: self.stride(),
            height: self.height(),
        }
    }

    fn rows(&self) -> FieldRows<Self::Item> {
        FieldRows(FieldView::new(
            self.data(),
            self.width(),
            self.stride(),
            self.height(),
        ))
    }

    fn cols(&self) -> FieldCols<Self::Item> {
        FieldCols(FieldView::new(
            self.data(),
            self.width(),
            self.stride(),
            self.height(),
        ))
    }

    fn offsets(&self) -> impl Iterator<Item = usize> + 'static {
        let (w, h, s) = (self.width(), self.height(), self.stride());
        (0..h).flat_map(move |y| (0..w).map(move |x| y * s + x))
    }

    fn coords<I: FromPrimitive<usize>>(&self) -> impl Iterator<Item = Coord<I>> + 'static {
        let (w, h) = (self.width(), self.height());
        (0..h).flat_map(move |y| (0..w).map(move |x| coord(I::from(x), I::from(y))))
    }
}

pub trait FieldMut: Field {
    fn data_mut(&mut self) -> &mut [Self::Item];

    fn get_mut<I: PrimitiveInt + ToPrimitive<usize>>(&mut self, x: I, y: I) -> &mut Self::Item {
        let o = self.offset(x, y);
        &mut self.data_mut()[o]
    }
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

    pub fn to_owned(self) -> FieldMutView<'static, T>
    where
        T: Clone,
    {
        FieldMutView::create_from_clone(self.data, self.width, self.stride, self.height)
    }
}

impl<'a, T> Field for FieldView<'a, T> {
    type Item = T;

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn stride(&self) -> usize {
        self.stride
    }

    fn data(&self) -> &[T] {
        self.data
    }
}

impl<'a, T> Clone for FieldView<'a, T> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<'a, T, I: PrimitiveInt + ToPrimitive<usize>> Index<I> for FieldView<'a, T> {
    type Output = T;
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index.to()]
    }
}

impl<'a, T, I: PrimitiveInt + ToPrimitive<usize>> Index<(I, I)> for FieldView<'a, T> {
    type Output = T;
    fn index(&self, pos: (I, I)) -> &Self::Output {
        &self.data[self.offset(pos.0, pos.1)]
    }
}

impl<'a, T, I: PrimitiveInt + ToPrimitive<usize>> Index<Coord<I>> for FieldView<'a, T> {
    type Output = T;
    fn index(&self, pos: Coord<I>) -> &Self::Output {
        &self.data[self.offset(pos.x, pos.y)]
    }
}

impl<'a> From<&'a [u8]> for FieldView<'a, u8> {
    fn from(input: &'a [u8]) -> Self {
        let b = input;
        let width = b.iter().position(|&c| c == b'\r' || c == b'\n').unwrap();
        let stride = width + 1 + ((b[width] == b'\r') as usize);
        let height = b.len().div_ceil(stride);
        Self::new(b, width, stride, height)
    }
}

impl<'a> From<&'a Input> for FieldView<'a, u8> {
    fn from(input: &'a Input) -> Self {
        FieldView::from(input.bytes())
    }
}

pub struct FieldMutView<'a, T> {
    data_ptr: *mut T,
    len: usize,
    width: usize,
    height: usize,
    stride: usize,
    #[allow(dead_code)] // `owned` is not read, but it is necessary to keep the data alive
    owned: Option<Box<[T]>>,
    _ref: PhantomData<&'a mut [T]>,
}

impl<'a, T> FieldMutView<'a, T> {
    pub fn from_ref(data: &'a mut [T], width: usize, stride: usize, height: usize) -> Self {
        Self {
            data_ptr: data.as_mut_ptr(),
            len: data.len(),
            width,
            height,
            stride,
            owned: None,
            _ref: PhantomData,
        }
    }

    pub fn to_owned(self) -> FieldMutView<'static, T>
    where
        T: Clone,
    {
        if self.owned.is_some() {
            // SAFETY: Since this is an owning FieldMutView, we know that the named lifetime 'a is actually just 'static
            unsafe { std::mem::transmute(self) }
        } else {
            FieldMutView::create_from_clone(self.data(), self.width, self.stride, self.height)
        }
    }
}

impl<T> FieldMutView<'static, T> {
    pub fn create_from_clone(data: &[T], width: usize, stride: usize, height: usize) -> Self
    where
        T: Clone,
    {
        let mut owned = data.to_vec().into_boxed_slice();
        Self {
            data_ptr: owned.as_mut_ptr(),
            len: data.len(),
            width,
            height,
            stride,
            owned: Some(owned),
            _ref: PhantomData,
        }
    }

    pub fn create_with_value(fill: T, width: usize, stride: usize, height: usize) -> Self
    where
        T: Clone,
    {
        let len = stride * height;
        let mut owned = vec![fill; len].into_boxed_slice();
        Self {
            data_ptr: owned.as_mut_ptr(),
            len,
            width,
            height,
            stride,
            owned: Some(owned),
            _ref: PhantomData,
        }
    }

    pub fn create_with_default(width: usize, stride: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self::create_with(width, stride, height, Default::default)
    }

    pub fn create_with(width: usize, stride: usize, height: usize, f: impl FnMut() -> T) -> Self {
        let len = stride * height;
        let mut owned = {
            let mut v = Vec::<T>::new();
            v.resize_with(len, f);
            v.into_boxed_slice()
        };
        Self {
            data_ptr: owned.as_mut_ptr(),
            len,
            width,
            height,
            stride,
            owned: Some(owned),
            _ref: PhantomData,
        }
    }
}

impl<'a, T> Field for FieldMutView<'a, T> {
    type Item = T;

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn stride(&self) -> usize {
        self.stride
    }

    fn data<'s>(&'s self) -> &'s [T] {
        unsafe { std::slice::from_raw_parts(self.data_ptr, self.len) }
    }
}

impl<'a, T> FieldMut for FieldMutView<'a, T> {
    // fn data_mut<'s>(&'s mut self) -> &'s mut [T] {
    fn data_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.data_ptr, self.len) }
    }
}

impl<'a, T, I: PrimitiveInt + ToPrimitive<usize>> Index<I> for FieldMutView<'a, T> {
    type Output = T;
    fn index(&self, index: I) -> &Self::Output {
        &self.data()[index.to()]
    }
}

impl<'a, T, I: PrimitiveInt + ToPrimitive<usize>> Index<(I, I)> for FieldMutView<'a, T> {
    type Output = T;
    fn index(&self, pos: (I, I)) -> &Self::Output {
        &self.data()[self.offset(pos.0, pos.1)]
    }
}

impl<'a, T, I: PrimitiveInt + ToPrimitive<usize>> Index<Coord<I>> for FieldMutView<'a, T> {
    type Output = T;
    fn index(&self, pos: Coord<I>) -> &Self::Output {
        &self.data()[self.offset(pos.x, pos.y)]
    }
}

impl<'a, T, I: PrimitiveInt + ToPrimitive<usize>> IndexMut<I> for FieldMutView<'a, T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.data_mut()[index.to()]
    }
}

impl<'a, T, I: PrimitiveInt + ToPrimitive<usize>> IndexMut<(I, I)> for FieldMutView<'a, T> {
    fn index_mut(&mut self, pos: (I, I)) -> &mut Self::Output {
        let o = self.offset(pos.0, pos.1);
        &mut self.data_mut()[o]
    }
}

impl<'a, T, I: PrimitiveInt + ToPrimitive<usize>> IndexMut<Coord<I>> for FieldMutView<'a, T> {
    fn index_mut(&mut self, pos: Coord<I>) -> &mut Self::Output {
        let o = self.offset(pos.x, pos.y);
        &mut self.data_mut()[o]
    }
}

impl<'a> From<&'a mut [u8]> for FieldMutView<'a, u8> {
    fn from(input: &'a mut [u8]) -> Self {
        let b = input;
        let width = b.iter().position(|&c| c == b'\r' || c == b'\n').unwrap();
        let stride = width + 1 + ((b[width] == b'\r') as usize);
        let height = b.len().div_ceil(stride);
        Self::from_ref(b, width, stride, height)
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

    pub fn get<I: PrimitiveInt + ToPrimitive<usize>>(&self, x: I, y: I) -> &T {
        self.view.get_or(x, y, &self.border)
    }
}

impl<'a, T> Field for BorderedFieldView<'a, T> {
    type Item = T;

    fn width(&self) -> usize {
        self.view.width
    }

    fn height(&self) -> usize {
        self.view.height
    }

    fn stride(&self) -> usize {
        self.view.stride
    }

    fn data(&self) -> &[T] {
        self.view.data
    }

    fn get<I: PrimitiveInt + ToPrimitive<usize>>(&self, x: I, y: I) -> &Self::Item {
        self.get(x, y)
    }
}

impl<'a, T, I: PrimitiveInt + ToPrimitive<usize>> Index<I> for BorderedFieldView<'a, T> {
    type Output = T;
    fn index(&self, index: I) -> &Self::Output {
        self.view.get_by_offset_or(index.to(), &self.border)
    }
}

impl<'a, T, I: PrimitiveInt + ToPrimitive<isize> + ToPrimitive<usize>> Index<(I, I)>
    for BorderedFieldView<'a, T>
{
    type Output = T;
    fn index(&self, pos: (I, I)) -> &Self::Output {
        self.view.get_or(pos.0, pos.1, &self.border)
    }
}

impl<'a, T, I: PrimitiveInt + ToPrimitive<isize> + ToPrimitive<usize>> Index<Coord<I>>
    for BorderedFieldView<'a, T>
{
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
