//! Tensor layout in bytes; views change layout only, never data.

use super::dtype::DType;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Layout {
    pub shape: Vec<usize>,
    /// Byte strides.
    pub strides: Vec<usize>,
    pub dtype: DType,
}

impl Layout {
    pub fn contiguous(shape: &[usize], dtype: DType) -> Self {
        let mut strides = vec![0; shape.len()];
        let mut acc = dtype.size_bytes();
        for (s, &dim) in strides.iter_mut().zip(shape).rev() {
            *s = acc;
            acc *= dim.max(1);
        }
        Self { shape: shape.to_vec(), strides, dtype }
    }

    pub fn elem_count(&self) -> usize {
        self.shape.iter().product()
    }

    pub fn byte_len(&self) -> usize {
        self.elem_count() * self.dtype.size_bytes()
    }

    pub fn is_contiguous(&self) -> bool {
        *self == Self::contiguous(&self.shape, self.dtype)
    }

    pub fn transpose(&self, a: usize, b: usize) -> Self {
        let mut l = self.clone();
        l.shape.swap(a, b);
        l.strides.swap(a, b);
        l
    }

    pub fn narrow(&self, dim: usize, len: usize) -> Self {
        let mut l = self.clone();
        l.shape[dim] = len;
        l
    }
}

#[cfg(test)]
#[path = "test/layout.rs"]
mod tests;
