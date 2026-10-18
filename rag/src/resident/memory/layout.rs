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
mod tests {
    use super::*;

    #[test]
    fn contiguous_byte_strides() {
        let l = Layout::contiguous(&[2, 3, 4], DType::F16);
        assert_eq!(l.strides, vec![24, 8, 2]);
        assert_eq!(l.byte_len(), 48);
        assert!(l.is_contiguous());
    }

    #[test]
    fn views_do_not_copy() {
        let l = Layout::contiguous(&[2, 3], DType::F32);
        let t = l.transpose(0, 1);
        assert_eq!(t.shape, vec![3, 2]);
        assert_eq!(t.strides, vec![4, 12]);
        assert!(!t.is_contiguous());
        assert_eq!(l.narrow(1, 2).shape, vec![2, 2]);
    }
}
