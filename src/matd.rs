//! The matrix type.

use crate::common::*;

/// The wrapper type of a matrix reference.
pub struct MatdRef<'a> {
    pub(crate) ref_: &'a sys::matd_t,
}

impl<'a> MatdRef<'a> {
    /// Get number of rows.
    pub fn nrows(&self) -> usize {
        self.ref_.nrows as usize
    }

    /// Get number of columns.
    pub fn ncols(&self) -> usize {
        self.ref_.ncols as usize
    }

    /// Get the reference to the matrix data.
    ///
    /// The values are in row-major order.
    pub fn data(&self) -> &'a [f64] {
        let len = self.nrows() * self.ncols();
        let data = unsafe { self.ref_.data.as_slice(len) };
        data
    }

    pub(crate) unsafe fn from_ptr(ptr: *const sys::matd_t) -> Self {
        Self {
            ref_: ptr.as_ref().expect("please report bug"),
        }
    }
}

impl<'a> Debug for MatdRef<'a> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let ncols = self.ncols();
        let nrows = self.nrows();
        let data = self.data();
        let mut list = formatter.debug_list();

        for row in 0..nrows {
            let begin = row * ncols;
            let end = begin + ncols;
            let slice = &data[begin..end];
            list.entry(&slice);
        }

        list.finish()
    }
}

#[cfg(feature = "nalgebra")]
mod nalgebra_conv {
    use super::*;
    use nalgebra::{DMatrix, DMatrixSlice};

    impl<'a> From<MatdRef<'a>> for DMatrix<f64> {
        fn from(from: MatdRef<'a>) -> Self {
            let nrows = from.nrows();
            let ncols = from.ncols();
            let data = from.data();
            DMatrixSlice::from_slice_with_strides(data, nrows, ncols, ncols, 1).transpose()
        }
    }
}
