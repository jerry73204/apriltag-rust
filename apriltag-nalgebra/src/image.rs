use apriltag::{image_buf::DEFAULT_ALIGNMENT_U8, Image};
use nalgebra::{DMatrix, Dim, Matrix, Storage};

pub trait ImageExt {
    fn to_na(&self) -> DMatrix<u8>;
    fn from_na<R, C, S>(matrix: &Matrix<u8, R, C, S>) -> Self
    where
        R: Dim,
        C: Dim,
        S: Storage<u8, R, C>;
}

impl ImageExt for Image {
    fn to_na(&self) -> DMatrix<u8> {
        let width = self.width();
        let height = self.height();
        DMatrix::from_fn(height, width, |row, col| self[(col, row)])
    }

    fn from_na<R, C, S>(matrix: &Matrix<u8, R, C, S>) -> Self
    where
        R: Dim,
        C: Dim,
        S: Storage<u8, R, C>,
    {
        let width = matrix.ncols();
        let height = matrix.nrows();
        let mut to = Image::zeros_with_alignment(width, height, DEFAULT_ALIGNMENT_U8).unwrap();

        for (row_idx, row) in matrix.row_iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                to[(col_idx, row_idx)] = value;
            }
        }

        to
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::SMatrix;

    #[test]
    fn na_matrix_vs_image_conversion() {
        let matrix_from = SMatrix::<u8, 80, 40>::new_random();

        let image = Image::from_na(&matrix_from);
        assert_eq!(matrix_from.nrows(), image.height());
        assert_eq!(matrix_from.ncols(), image.width());
        assert!({
            image
                .indexed_samples_iter()
                .all(|(x, y, value)| value == matrix_from[(y, x)])
        });

        let matrix_to = image.to_na();
        assert_eq!(matrix_from.nrows(), matrix_to.nrows());
        assert_eq!(matrix_from.ncols(), matrix_to.ncols());
        assert!({
            matrix_from
                .iter()
                .zip(matrix_to.iter())
                .all(|(lhs, rhs)| lhs == rhs)
        });
    }
}
