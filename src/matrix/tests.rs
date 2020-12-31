use super::*;

mod unit_tests {
    use super::*;
    use std::f64::consts::PI;

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
    fn multiplying_a_point_by_a_translation_matrix_should_move_the_point_by_the_provided_x_y_and_z()
    {
        let point = Point3D::new(-3.0, 4.0, 5.0);
        let translation = Matrix4D::translation(5.0, -3.0, 2.0);

        let translated = translation * point;
        assert_eq!(translated, (2.0, 1.0, 7.0, 1.0));
    }

    #[test]
    fn multiplying_an_inverted_translation_matrix_by_a_point_should_move_the_point_by_negative_x_y_and_z(
    ) {
        let point = Point3D::new(-3.0, 4.0, 5.0);
        let translation = Matrix4D::translation(5.0, -3.0, 2.0);

        let translated = translation.inverse().unwrap() * point;
        assert_eq!(translated, (-8.0, 7.0, 3.0, 1.0));
    }

    #[test]
    fn multiplying_a_translation_matrix_by_a_vector_should_produce_the_same_vector() {
        let vector = Vector3D::new(-3.0, 4.0, 5.0);
        let translation = Matrix4D::translation(5.0, -3.0, 2.0);

        let translated = translation * vector;
        assert_eq!(translated, (-3.0, 4.0, 5.0, 0.0));
    }

    #[test]
    fn multiplying_a_scaling_matrix_by_a_point_should_scale_x_y_and_z_components() {
        let point = Point3D::new(-4.0, 6.0, 8.0);
        let scale = Matrix4D::scaling(2.0, 3.0, 4.0);

        let scaled = scale * point;
        assert_eq!(scaled, (-8.0, 18.0, 32.0, 1.0));
    }

    #[test]
    fn multiplying_a_scaling_matrix_by_a_vector_should_scale_x_y_and_z_components() {
        let vector = Vector3D::new(-4.0, 6.0, 8.0);
        let scale = Matrix4D::scaling(2.0, 3.0, 4.0);

        let scaled = scale * vector;
        assert_eq!(scaled, (-8.0, 18.0, 32.0, 0.0));
    }

    #[test]
    fn multiplying_an_inverted_scaling_matrix_by_a_vector_should_scale_down_x_y_and_z_components() {
        let vector = Vector3D::new(-4.0, 6.0, 8.0);
        let scale = Matrix4D::scaling(2.0, 3.0, 4.0);

        let scaled = scale.inverse().unwrap() * vector;
        assert_eq!(scaled, (-2.0, 2.0, 2.0, 0.0));
    }

    #[test]
    fn should_be_able_to_rotate_a_point_around_x_axis() {
        let point = Point3D::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix4D::rotation_x(PI / 4.0);
        let full_quarter = Matrix4D::rotation_x(PI / 2.0);

        {
            let (x, y, z, w) = half_quarter * point;
            assert_eq!(x, 0.0);
            assert!(approx_eq!(f64, y, 2.0_f64.sqrt() / 2.0));
            assert!(approx_eq!(f64, z, 2.0_f64.sqrt() / 2.0));
            assert_eq!(w, 1.0);
        }

        {
            let (x, y, z, w) = full_quarter * point;
            assert_eq!(x, 0.0);
            assert!(approx_eq!(f64, y, 0.0));
            assert!(approx_eq!(f64, z, 1.0));
            assert_eq!(w, 1.0);
        }
    }

    #[test]
    fn should_be_able_to_rotate_around_the_y_axis() {
        let point = Point3D::new(0.0, 0.0, 1.0);
        let half_quarter = Matrix4D::rotation_y(PI / 4.0);
        let full_quarter = Matrix4D::rotation_y(PI / 2.0);

        {
            let (x, y, z, w) = half_quarter * point;
            assert!(approx_eq!(f64, x, 2.0_f64.sqrt() / 2.0));
            assert_eq!(y, 0.0);
            assert!(approx_eq!(f64, z, 2.0_f64.sqrt() / 2.0));
            assert_eq!(w, 1.0);
        }

        {
            let (x, y, z, w) = full_quarter * point;
            assert!(approx_eq!(f64, x, 1.0));
            assert_eq!(y, 0.0);
            assert!(approx_eq!(f64, z, 0.0));
            assert_eq!(w, 1.0);
        }
    }

    #[test]
    fn should_be_able_to_rotate_around_the_z_axis() {
        let point = Point3D::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix4D::rotation_z(PI / 4.0);
        let full_quarter = Matrix4D::rotation_z(PI / 2.0);

        {
            let (x, y, z, w) = half_quarter * point;
            assert!(approx_eq!(f64, x, -(2.0_f64.sqrt() / 2.0)));
            assert!(approx_eq!(f64, y, 2.0_f64.sqrt() / 2.0));
            assert_eq!(z, 0.0);
            assert_eq!(w, 1.0);
        }

        {
            let (x, y, z, w) = full_quarter * point;
            assert!(approx_eq!(f64, x, -1.0));
            assert!(approx_eq!(f64, y, 0.0));
            assert_eq!(z, 0.0);
            assert_eq!(w, 1.0);
        }
    }

    #[test]
    fn rotating_a_point_around_an_inverted_rotation_matrix_rotates_in_the_opposite_direction() {
        let point = Point3D::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix4D::rotation_x(PI / 4.0);

        {
            let (x, y, z, w) = half_quarter.inverse().unwrap() * point;
            assert_eq!(x, 0.0);
            assert!(approx_eq!(f64, y, 2.0_f64.sqrt() / 2.0));
            assert!(approx_eq!(f64, z, -(2.0_f64.sqrt() / 2.0)));
            assert_eq!(w, 1.0);
        }
    }

    #[test]
    fn an_x_to_y_shear_moves_x_in_proportion_to_y() {
        let point = Point3D::new(2.0, 3.0, 4.0);
        let shear = Matrix4D::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

        let sheared = shear * point;
        assert_eq!(sheared, (5.0, 3.0, 4.0, 1.0));
    }

    #[test]
    fn an_x_to_z_shear_moves_x_in_proportion_to_z() {
        let point = Point3D::new(2.0, 3.0, 4.0);
        let shear = Matrix4D::shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);

        let sheared = shear * point;
        assert_eq!(sheared, (6.0, 3.0, 4.0, 1.0));
    }

    #[test]
    fn a_y_to_x_shear_moves_y_in_proportion_to_x() {
        let point = Point3D::new(2.0, 3.0, 4.0);
        let shear = Matrix4D::shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

        let sheared = shear * point;
        assert_eq!(sheared, (2.0, 5.0, 4.0, 1.0));
    }

    #[test]
    fn a_y_to_z_shear_moves_y_in_proportion_to_z() {
        let point = Point3D::new(2.0, 3.0, 4.0);
        let shear = Matrix4D::shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        let sheared = shear * point;
        assert_eq!(sheared, (2.0, 7.0, 4.0, 1.0));
    }

    #[test]
    fn a_z_to_x_shear_moves_z_in_proportion_to_x() {
        let point = Point3D::new(2.0, 3.0, 4.0);
        let shear = Matrix4D::shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);

        let sheared = shear * point;
        assert_eq!(sheared, (2.0, 3.0, 6.0, 1.0));
    }

    #[test]
    fn a_z_to_y_shear_moves_z_in_proportion_to_y() {
        let point = Point3D::new(2.0, 3.0, 4.0);
        let shear = Matrix4D::shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        let sheared = shear * point;
        assert_eq!(sheared, (2.0, 3.0, 7.0, 1.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let point = Point3D::new(1.0, 0.0, 1.0);
        let rotation = Matrix4D::rotation_x(PI / 2.0);
        let scale = Matrix4D::scaling(5.0, 5.0, 5.0);
        let translation = Matrix4D::translation(10.0, 5.0, 7.0);

        let rotated = rotation * point;
        {
            let (x, y, z, _) = rotated;

            assert!(approx_eq!(f64, x, 1.0));
            assert!(approx_eq!(f64, y, -1.0));
            assert!(approx_eq!(f64, z, 0.0));
        }

        let scaled = scale * rotated;
        {
            let (x, y, z, _) = scaled;

            assert!(approx_eq!(f64, x, 5.0));
            assert!(approx_eq!(f64, y, -5.0));
            assert!(approx_eq!(
                f64,
                z,
                0.0,
                ulps = 5,
                epsilon = f32::EPSILON as f64
            ));
        }

        let translated = translation * scaled;
        {
            let (x, y, z, _) = translated;

            assert!(approx_eq!(f64, x, 15.0));
            assert!(approx_eq!(f64, y, 0.0));
            assert!(approx_eq!(f64, z, 7.0));
        }
    }

    #[test]
    fn combined_transformations_are_applied_in_reverse_order() {
        let point = Point3D::new(1.0, 0.0, 1.0);
        let rotation = Matrix4D::rotation_x(PI / 2.0);
        let scale = Matrix4D::scaling(5.0, 5.0, 5.0);
        let translation = Matrix4D::translation(10.0, 5.0, 7.0);

        let transform = translation * scale * rotation;
        let (x, y, z, w) = transform * point;

        assert_eq!(x, 15.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 7.0);
        assert_eq!(w, 1.0);
    }

    #[test]
    fn fluent_api_applies_transformations_in_sequence() {
        let point = Point3D::new(1.0, 0.0, 1.0);

        let translation = Matrix4D::identity()
            .with_rotation_x(PI / 2.0)
            .with_scaling(5.0, 5.0, 5.0)
            .with_translation(10.0, 5.0, 7.0);

        assert_eq!(translation * point, (15.0, 0.0, 7.0, 1.0));
    }

    #[test]
    fn the_view_transform_of_the_default_orientation_is_the_identity_matrix() {
        let transform = Matrix4D::view_transform(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.0, 0.0, -1.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );

        assert_eq!(transform, Matrix4D::identity());
    }

    #[test]
    fn the_view_transform_for_a_positive_z_orientation_is_a_scaling_matrix() {
        let transform = Matrix4D::view_transform(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );

        assert_eq!(transform, Matrix4D::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn a_view_translation_moves_the_world_in_the_opposite_direction() {
        let transform = Matrix4D::view_transform(
            Point3D::new(0.0, 0.0, 8.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );

        assert_eq!(transform, Matrix4D::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn an_arbitrary_orientation_produces_the_correct_view_transform() {
        let transform = Matrix4D::view_transform(
            Point3D::new(1.0, 3.0, 2.0),
            Point3D::new(4.0, -2.0, 8.0),
            Vector3D::new(1.0, 1.0, 0.0),
        );

        #[rustfmt::skip]
        assert_eq!(
            transform,
            Matrix4D::new(
                [-0.5070925528371099, 0.5070925528371099, 0.6761234037828132, -2.366431913239846],
                [0.7677159338596801, 0.6060915267313263, 0.12121830534626524, -2.8284271247461894],
                [-0.35856858280031806, 0.5976143046671968, -0.7171371656006361, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            )
        );
    }
}

mod property_tests {
    extern crate float_cmp;
    extern crate quickcheck;
    use super::*;

    #[quickcheck]
    fn multiplying_a_vector_by_identity_matrix_produces_a_4_tuple_of_the_vector_components(
        vector: Vector3D,
    ) {
        assert_eq!(
            Matrix4D::identity() * vector,
            (vector.x(), vector.y(), vector.z(), vector.w())
        );
    }

    #[quickcheck]
    fn multiplying_a_point_by_identity_matrix_produces_a_4_tuple_of_the_point_components(
        point: Point3D,
    ) {
        assert_eq!(
            Matrix4D::identity() * point,
            (point.x(), point.y(), point.z(), point.w())
        );
    }

    #[quickcheck]
    fn multiplying_a_matrix_by_a_matrix_inverse_undoes_multiplication(
        first: Matrix4D,
        second: Matrix4D,
    ) {
        // rounding errors become significant
        fn assert_close_enough(f: f64, s: f64) {
            assert!(
                approx_eq!(f64, f, s, epsilon = f32::EPSILON as f64),
                "not approximately equal: {} != {}",
                f,
                s
            )
        }

        if second.determinant() != 0.0 {
            let inverse = second.inverse();
            assert!(inverse.is_some());
            let inverse = inverse.unwrap();

            let product = (first.clone() * second) * inverse;
            assert_close_enough(first.m00(), product.m00());
            assert_close_enough(first.m01(), product.m01());
            assert_close_enough(first.m02(), product.m02());
            assert_close_enough(first.m03(), product.m03());
            assert_close_enough(first.m10(), product.m10());
            assert_close_enough(first.m11(), product.m11());
            assert_close_enough(first.m12(), product.m12());
            assert_close_enough(first.m13(), product.m13());
            assert_close_enough(first.m20(), product.m20());
            assert_close_enough(first.m21(), product.m21());
            assert_close_enough(first.m22(), product.m22());
            assert_close_enough(first.m23(), product.m23());
            assert_close_enough(first.m30(), product.m30());
            assert_close_enough(first.m31(), product.m31());
            assert_close_enough(first.m32(), product.m32());
            assert_close_enough(first.m33(), product.m33());
        }
    }

    #[quickcheck]
    fn vectors_cannot_be_translated(vector: Vector3D, x: f64, y: f64, z: f64) {
        let translation = Matrix4D::translation(x, y, z);

        assert_eq!(
            translation * vector,
            (vector.x(), vector.y(), vector.z(), 0.0)
        );
    }

    #[quickcheck]
    fn fluent_translate_api_behaves_the_same_as_translation_matrix(
        point: Point3D,
        x: f64,
        y: f64,
        z: f64,
    ) {
        let direct = Matrix4D::translation(x, y, z);
        let fluent = Matrix4D::identity().with_translation(x, y, z);

        assert_eq!(direct * point, fluent * point);
    }

    #[quickcheck]
    fn fluent_scaling_api_behaves_the_same_as_scaling_matrix(
        point: Point3D,
        x: f64,
        y: f64,
        z: f64,
    ) {
        let direct = Matrix4D::scaling(x, y, z);
        let fluent = Matrix4D::identity().with_scaling(x, y, z);

        assert_eq!(direct * point, fluent * point);
    }

    #[quickcheck]
    fn fluent_rotation_x_api_behaves_the_same_as_rotation_x_matrix(point: Point3D, radians: f64) {
        let direct = Matrix4D::rotation_x(radians);
        let fluent = Matrix4D::identity().with_rotation_x(radians);

        assert_eq!(direct * point, fluent * point);
    }

    #[quickcheck]
    fn fluent_rotation_y_api_behaves_the_same_as_rotation_y_matrix(point: Point3D, radians: f64) {
        let direct = Matrix4D::rotation_y(radians);
        let fluent = Matrix4D::identity().with_rotation_y(radians);

        assert_eq!(direct * point, fluent * point);
    }

    #[quickcheck]
    fn fluent_rotation_z_api_behaves_the_same_as_rotation_z_matrix(point: Point3D, radians: f64) {
        let direct = Matrix4D::rotation_z(radians);
        let fluent = Matrix4D::identity().with_rotation_z(radians);

        assert_eq!(direct * point, fluent * point);
    }

    #[quickcheck]
    fn fluent_shear_api_behaves_the_same_as_shear_matrix(
        point: Point3D,
        x_to_y: f64,
        x_to_z: f64,
        y_to_x: f64,
        y_to_z: f64,
        z_to_x: f64,
        z_to_y: f64,
    ) {
        let direct = Matrix4D::shear(x_to_y, x_to_z, y_to_x, y_to_z, z_to_x, z_to_y);
        let fluent =
            Matrix4D::identity().with_shear(x_to_y, x_to_z, y_to_x, y_to_z, z_to_x, z_to_y);

        assert_eq!(direct * point, fluent * point);
    }
}
