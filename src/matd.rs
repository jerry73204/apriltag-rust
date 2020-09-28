//! The matrix type.

use apriltag_sys as sys;

/// The wrapper type of a matrix reference.
#[derive(Debug)]
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

#[cfg(feature = "nalgebra")]
mod nalgebra_conv {
    use super::*;
    use nalgebra::{base::dimension::Dynamic, DMatrixSlice};

    impl<'a> From<MatdRef<'a>> for DMatrixSlice<'a, f64, Dynamic, Dynamic> {
        fn from(from: MatdRef<'a>) -> Self {
            let nrows = from.nrows();
            let ncols = from.ncols();
            let data = from.data();
            Self::from_slice_with_strides(data, nrows, ncols, ncols, 1)
        }
    }
}
