use super::*;

#[test]
fn should_be_able_to_create_4d_matrix() {
    let matrix4d = Matrix4D::new(
        [1.0, 2.0, 3.0, 4.0],
        [5.5, 6.5, 7.5, 8.5],
        [9.0, 10.0, 11.0, 12.0],
        [13.5, 14.5, 15.5, 16.5],
    );

    assert_eq!(matrix4d.m00(), 1.0);
    assert_eq!(matrix4d.m03(), 4.0);
    assert_eq!(matrix4d.m10(), 5.5);
    assert_eq!(matrix4d.m12(), 7.5);
    assert_eq!(matrix4d.m22(), 11.0);
    assert_eq!(matrix4d.m30(), 13.5);
    assert_eq!(matrix4d.m32(), 15.5);
}

#[test]
fn should_be_able_to_multiply_4d_matrices() {
    let m1 = Matrix4D::new(
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    );
    let m2 = Matrix4D::new(
        [-2.0, 1.0, 2.0, 3.0],
        [3.0, 2.0, 1.0, -1.0],
        [4.0, 3.0, 6.0, 5.0],
        [1.0, 2.0, 7.0, 8.0],
    );
    let product = Matrix4D::new(
        [20.0, 22.0, 50.0, 48.0],
        [44.0, 54.0, 114.0, 108.0],
        [40.0, 58.0, 110.0, 102.0],
        [16.0, 26.0, 46.0, 42.0],
    );

    assert_eq!(m1 * m2, product);
}

#[test]
fn should_be_able_to_multiply_a_matrix_by_a_point() {
    let matrix = Matrix4D::new(
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    );

    let point = Point3D::new(1.0, 2.0, 3.0);

    assert_eq!(matrix * point, (18.0, 24.0, 33.0, 1.0));
}

#[test]
fn multiplying_a_matrix_by_identity_produces_the_same_matrix() {
    let matrix = Matrix4D::new(
        [0.0, 1.0, 2.0, 4.0],
        [1.0, 2.0, 4.0, 8.0],
        [2.0, 4.0, 8.0, 16.0],
        [4.0, 8.0, 16.0, 32.0],
    );

    assert_eq!(matrix.clone() * Matrix4D::identity(), matrix);
}

#[test]
fn transposing_a_matrix_rotates_the_rows_and_columns() {
    let matrix = Matrix4D::new(
        [0.0, 9.0, 3.0, 0.0],
        [9.0, 8.0, 0.0, 8.0],
        [1.0, 8.0, 5.0, 3.0],
        [0.0, 0.0, 5.0, 8.0],
    );

    let transposed = Matrix4D::new(
        [0.0, 9.0, 1.0, 0.0],
        [9.0, 8.0, 8.0, 0.0],
        [3.0, 0.0, 5.0, 5.0],
        [0.0, 8.0, 3.0, 8.0],
    );

    assert_eq!(matrix.transpose(), transposed);
}

#[test]
fn should_be_able_to_take_a_3d_submatrix_of_a_4d_matrix() {
    let matrix_4d = Matrix4D::new(
        [-6.0, 1.0, 1.0, 6.0],
        [-8.0, 5.0, 8.0, 6.0],
        [-1.0, 0.0, 8.0, 2.0],
        [-7.0, 1.0, -1.0, 1.0],
    );

    #[rustfmt::skip]
        let submatrix_3d = Matrix3D::new(
        [-6.0, 1.0, 6.0],
        [-8.0, 8.0, 6.0],
        [-7.0, -1.0, 1.0]
    );

    assert_eq!(matrix_4d.submatrix(2, 1), submatrix_3d);
}

#[test]
fn should_be_able_to_calculate_the_determinant_of_a_4d_matrix() {
    let matrix_4d = Matrix4D::new(
        [-2.0, -8.0, 3.0, 5.0],
        [-3.0, 1.0, 7.0, 3.0],
        [1.0, 2.0, -9.0, 6.0],
        [-6.0, 7.0, 7.0, -9.0],
    );

    assert_eq!(matrix_4d.cofactor(0, 0), 690.0);
    assert_eq!(matrix_4d.cofactor(0, 1), 447.0);
    assert_eq!(matrix_4d.cofactor(0, 2), 210.0);
    assert_eq!(matrix_4d.cofactor(0, 3), 51.0);
    assert_eq!(matrix_4d.determinant(), -4071.0);
}

#[test]
fn a_4d_matrix_with_a_non_zero_determinant_is_invertible() {
    let matrix = Matrix4D::new(
        [6.0, 4.0, 4.0, 4.0],
        [5.0, 5.0, 7.0, 6.0],
        [4.0, -9.0, 3.0, -7.0],
        [9.0, 1.0, 7.0, -6.0],
    );

    assert_eq!(matrix.determinant(), -2120.0);
    assert!(matrix.inverse().is_some());
}

#[test]
fn a_4d_matrix_with_a_zero_determinant_is_not_invertible() {
    let matrix = Matrix4D::new(
        [-4.0, 2.0, -2.0, -3.0],
        [9.0, 6.0, 2.0, 6.0],
        [0.0, -5.0, 1.0, -5.0],
        [0.0, 0.0, 0.0, 0.0],
    );

    assert_eq!(matrix.determinant(), 0.0);
    assert!(matrix.inverse().is_none());
}

#[test]
fn should_be_able_to_calculate_the_inverse_of_a_4d_matrix() {
    let matrix = Matrix4D::new(
        [-5.0, 2.0, 6.0, -8.0],
        [1.0, -5.0, 1.0, 8.0],
        [7.0, 7.0, -6.0, -7.0],
        [1.0, -3.0, 7.0, 4.0],
    );

    let inverse = matrix.inverse();
    assert!(inverse.is_some());
    let inverse = inverse.unwrap();

    assert_eq!(matrix.determinant(), 532.0);
    assert_eq!(matrix.cofactor(2, 3), -160.0);
    assert_eq!(inverse.m32(), -160.0 / 532.0);

    assert_eq!(matrix.cofactor(3, 2), 105.0);
    assert_eq!(inverse.m23(), 105.0 / 532.0);

    #[rustfmt::skip]
    assert_eq!(
        inverse,
        Matrix4D::new(
            [0.21804511278195488, 0.45112781954887216, 0.24060150375939848, -0.045112781954887216],
            [-0.8082706766917294, -1.4567669172932332, -0.44360902255639095, 0.5206766917293233],
            [-0.07894736842105263, -0.2236842105263158, -0.05263157894736842, 0.19736842105263158],
            [-0.5225563909774437, -0.8139097744360902, -0.3007518796992481, 0.30639097744360905],
        )
    );
}

#[test]
fn should_be_able_to_calculate_the_inverse_of_other_4d_matrices() {
    let first = Matrix4D::new(
        [8.0, -5.0, 9.0, 2.0],
        [7.0, 5.0, 6.0, 1.0],
        [-6.0, 0.0, 9.0, 6.0],
        [-3.0, 0.0, -9.0, -4.0],
    );
    #[rustfmt::skip]
        let first_inverted = Matrix4D::new(
        [-0.15384615384615385, -0.15384615384615385, -0.28205128205128205, -0.5384615384615384],
        [-0.07692307692307693, 0.12307692307692308, 0.02564102564102564, 0.03076923076923077],
        [0.358974358974359, 0.358974358974359, 0.4358974358974359, 0.9230769230769231],
        [-0.6923076923076923, -0.6923076923076923, -0.7692307692307693, -1.9230769230769231],
    );

    assert_eq!(first.inverse().unwrap(), first_inverted);

    let second = Matrix4D::new(
        [9.0, 3.0, 0.0, 9.0],
        [-5.0, -2.0, -6.0, -3.0],
        [-4.0, 9.0, 6.0, 4.0],
        [-7.0, 6.0, 6.0, 2.0],
    );

    #[rustfmt::skip]
        let second_inverted = Matrix4D::new(
        [-0.040740740740740744, -0.07777777777777778,  0.14444444444444443, -0.2222222222222222],
        [-0.07777777777777778,   0.03333333333333333,  0.36666666666666664, -0.3333333333333333],
        [-0.029012345679012345, -0.14629629629629629, -0.10925925925925926,  0.12962962962962962],
        [ 0.17777777777777778,   0.06666666666666667, -0.26666666666666666,  0.3333333333333333],
    );

    assert_eq!(second.inverse().unwrap(), second_inverted);
}

#[test]
fn should_be_able_to_create_3d_matrix() {
    #[rustfmt::skip]
        let matrix3d = Matrix3D::new(
        [-3.0, 5.0, 0.0],
        [1.0, -2.0, -7.0],
        [0.0, 1.0, 1.0]
    );

    assert_eq!(matrix3d.m00(), -3.0);
    assert_eq!(matrix3d.m11(), -2.0);
    assert_eq!(matrix3d.m22(), 1.0);
}

#[test]
fn should_be_able_to_create_2d_matrix() {
    #[rustfmt::skip]
        let matrix2d = Matrix2D::new(
        [-3.0, 5.0],
        [1.0, -2.0],
    );

    assert_eq!(matrix2d.m00(), -3.0);
    assert_eq!(matrix2d.m01(), 5.0);
    assert_eq!(matrix2d.m10(), 1.0);
    assert_eq!(matrix2d.m11(), -2.0);
}

#[test]
fn should_be_able_to_calculate_determinant_of_a_2d_matrix() {
    let matrix = Matrix2D::new([1.0, 5.0], [-3.0, 2.0]);
    assert_eq!(matrix.determinant(), 17.0);
}

#[test]
fn should_be_able_to_take_a_2d_submatrix_of_a_3d_matrix() {
    #[rustfmt::skip]
        let matrix_3d = Matrix3D::new(
        [1.0, 5.0, 0.0],
        [-3.0, 2.0, 7.0],
        [0.0, 6.0, -3.0]
    );

    let submatrix_2d = Matrix2D::new([-3.0, 2.0], [0.0, 6.0]);

    assert_eq!(matrix_3d.submatrix(0, 2), submatrix_2d);
}

#[test]
fn the_minor_of_a_row_and_column_of_a_3d_matrix_should_be_the_determinant_of_the_submatrix_excluding_the_row_and_column(
) {
    #[rustfmt::skip]
        let matrix_3d = Matrix3D::new(
        [3.0, 5.0, 0.0],
        [2.0, -1.0, -7.0],
        [6.0, -1.0, 5.0]
    );

    assert_eq!(matrix_3d.submatrix(1, 0).determinant(), 25.0);
    assert_eq!(matrix_3d.minor(1, 0), 25.0);
}

#[test]
fn should_be_able_to_calculate_the_cofactor_of_a_3d_matrix_row_and_column() {
    #[rustfmt::skip]
        let matrix_3d = Matrix3D::new(
        [3.0, 5.0, 0.0],
        [2.0, -1.0, -7.0],
        [6.0, -1.0, 5.0]
    );

    assert_eq!(matrix_3d.minor(0, 0), -12.0);
    assert_eq!(matrix_3d.cofactor(0, 0), -12.0);
    assert_eq!(matrix_3d.minor(1, 0), 25.0);
    assert_eq!(matrix_3d.cofactor(1, 0), -25.0);
}

#[test]
fn should_be_able_to_calculate_the_determinant_of_a_3d_matrix() {
    #[rustfmt::skip]
        let matrix_3d = Matrix3D::new(
        [1.0, 2.0, 6.0],
        [-5.0, 8.0, -4.0],
        [2.0, 6.0, 4.0]
    );

    assert_eq!(matrix_3d.cofactor(0, 0), 56.0);
    assert_eq!(matrix_3d.cofactor(0, 1), 12.0);
    assert_eq!(matrix_3d.cofactor(0, 2), -46.0);
    assert_eq!(matrix_3d.determinant(), -196.0);
}
