#![allow(dead_code)]
use nalgebra::{Matrix2, Matrix3, Matrix3x2, RowVector3};

#[rustfmt::skip]
pub fn matrix3_to_covar(m: Matrix3<f64>) -> [f64; 9] {
    [
        m[(0,0)], m[(0,1)], m[(0,2)],
        m[(1,0)], m[(1,1)], m[(1,2)],
        m[(2,0)], m[(2,1)], m[(2,2)]
    ]
}

#[rustfmt::skip]
pub fn covar_to_matrix3(cov: [f64; 9]) -> Matrix3<f64> {
    Matrix3::from_rows(&[
        RowVector3::new(cov[0], cov[1], cov[2]),
        RowVector3::new(cov[3], cov[4], cov[5]),
        RowVector3::new(cov[6], cov[7], cov[8])
    ])
}

pub fn mat_multiply_3x2_2x2_2x3(jacob: Matrix3x2<f64>, covar: Matrix2<f64>) -> Matrix3<f64> {
    jacob * (covar * jacob.transpose())
}

pub fn mat_multiply_3x3_3x3_3x3(jacob: Matrix3<f64>, covar: Matrix3<f64>) -> Matrix3<f64> {
    jacob * (covar * jacob.transpose())
}
