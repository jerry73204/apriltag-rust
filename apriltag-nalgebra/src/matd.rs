use apriltag::MatdRef;
use nalgebra::{DMatrix, DMatrixView};

pub trait MatdRefExt {
    fn to_na(&self) -> DMatrix<f64>;
}

impl<'a> MatdRefExt for MatdRef<'a> {
    fn to_na(&self) -> DMatrix<f64> {
        let nrows = self.nrows();
        let ncols = self.ncols();
        let data = self.data();
        DMatrixView::from_slice_with_strides(data, nrows, ncols, ncols, 1).transpose()
    }
}
