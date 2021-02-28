extern crate autonomy;
extern crate nalgebra as na;
use autonomy::utils::matrix::{
    covar_to_matrix3, mat_multiply_3x2_2x2_2x3, mat_multiply_3x3_3x3_3x3, matrix3_to_covar,
};
use na::{Matrix2, Matrix3, Matrix3x2, RowVector2};

#[test]
fn test_matrix_multiply_3x2_2x2_2_3() {
    #[rustfmt::skip]
        let covar = Matrix2::from_rows(&[
        RowVector2::new(11.0, 12.0),
        RowVector2::new(21.0, 22.0)]
    );

    let jacob = Matrix3x2::from_rows(&[
        RowVector2::new(11.0, 12.0),
        RowVector2::new(21.0, 22.0),
        RowVector2::new(31.0, 32.0),
    ]);

    let m: Matrix3<f64> = mat_multiply_3x2_2x2_2x3(jacob, covar);
    let actual: [f64; 9] = matrix3_to_covar(m);
    let expected: [f64; 9] = [
        8855.0, 16545.0, 24235.0, 16455.0, 30745.0, 45035.0, 24055.0, 44945.0, 65835.0,
    ];

    assert_eq!(
        &expected[..],
        &actual[..],
        "\nExpected\n{:?}\nactual\n{:?}",
        &expected[..],
        &actual[..]
    );
}

#[test]
fn test_matrix_multiply_3x3_3x3_3_3() {
    let c1 = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    let c2 = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

    let m1 = covar_to_matrix3(c1);
    let m2 = covar_to_matrix3(c2);

    let m: Matrix3<f64> = mat_multiply_3x3_3x3_3x3(m1, m2);
    let actual: [f64; 9] = matrix3_to_covar(m);
    let expected: [f64; 9] = [36.0, 36.0, 36.0, 36.0, 36.0, 36.0, 36.0, 36.0, 36.0];

    assert_eq!(
        &expected[..],
        &actual[..],
        "\nExpected\n{:?}\nactual\n{:?}",
        &expected[..],
        &actual[..]
    );
}

#[test]
fn test_matrix_multiply_zero_init_covar() {
    let covar = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let jacob = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

    let m_covar = covar_to_matrix3(covar);
    let m_jacob = covar_to_matrix3(jacob);

    let m: Matrix3<f64> = mat_multiply_3x3_3x3_3x3(m_jacob, m_covar);
    let actual: [f64; 9] = matrix3_to_covar(m);
    let expected: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

    assert_eq!(
        &expected[..],
        &actual[..],
        "\nExpected\n{:?}\nactual\n{:?}",
        &expected[..],
        &actual[..]
    );
}
