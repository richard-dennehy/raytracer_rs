use super::*;

mod unit_tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn multiplying_a_point_by_a_translation_matrix_should_move_the_point_by_the_provided_x_y_and_z()
    {
        let point = Point3D::new(-3.0, 4.0, 5.0);
        let translation = Transform::translation(5.0, -3.0, 2.0);

        let translated = translation * point;
        assert_eq!(translated, Point3D::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_an_inverted_translation_matrix_by_a_point_should_move_the_point_by_negative_x_y_and_z(
    ) {
        let point = Point3D::new(-3.0, 4.0, 5.0);
        let translation = Transform::translation(5.0, -3.0, 2.0);

        let translated = translation.inverse().unwrap() * point;
        assert_eq!(translated, Point3D::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn multiplying_a_translation_matrix_by_a_vector_should_produce_the_same_vector() {
        let vector = Vector3D::new(-3.0, 4.0, 5.0);
        let translation = Transform::translation(5.0, -3.0, 2.0);

        let translated = translation * vector;
        assert_eq!(translated, Vector3D::new(-3.0, 4.0, 5.0));
    }

    #[test]
    fn multiplying_a_scaling_matrix_by_a_point_should_scale_x_y_and_z_components() {
        let point = Point3D::new(-4.0, 6.0, 8.0);
        let scale = Transform::scaling(2.0, 3.0, 4.0);

        let scaled = scale * point;
        assert_eq!(scaled, Point3D::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_a_scaling_matrix_by_a_vector_should_scale_x_y_and_z_components() {
        let vector = Vector3D::new(-4.0, 6.0, 8.0);
        let scale = Transform::scaling(2.0, 3.0, 4.0);

        let scaled = scale * vector;
        assert_eq!(scaled, Vector3D::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_an_inverted_scaling_matrix_by_a_vector_should_scale_down_x_y_and_z_components() {
        let vector = Vector3D::new(-4.0, 6.0, 8.0);
        let scale = Transform::scaling(2.0, 3.0, 4.0);

        let scaled = scale.inverse().unwrap() * vector;
        assert_eq!(scaled, Vector3D::new(-2.0, 2.0, 2.0));
    }

    #[test]
    fn should_be_able_to_rotate_a_point_around_x_axis() {
        let point = Point3D::new(0.0, 1.0, 0.0);
        let half_quarter = Transform::identity().rotate_x(PI / 4.0);
        let full_quarter = Transform::identity().rotate_x(PI / 2.0);

        {
            let point = half_quarter * point;
            assert_eq!(point.x(), 0.0);
            assert!(approx_eq!(f64, point.y(), 2.0_f64.sqrt() / 2.0));
            assert!(approx_eq!(f64, point.z(), 2.0_f64.sqrt() / 2.0));
        }

        {
            let point = full_quarter * point;
            assert_eq!(point.x(), 0.0);
            assert!(approx_eq!(f64, point.y(), 0.0));
            assert!(approx_eq!(f64, point.z(), 1.0));
        }
    }

    #[test]
    fn should_be_able_to_rotate_around_the_y_axis() {
        let point = Point3D::new(0.0, 0.0, 1.0);
        let half_quarter = Transform::identity().rotate_y(PI / 4.0);
        let full_quarter = Transform::identity().rotate_y(PI / 2.0);

        {
            let point = half_quarter * point;
            assert!(approx_eq!(f64, point.x(), 2.0_f64.sqrt() / 2.0));
            assert_eq!(point.y(), 0.0);
            assert!(approx_eq!(f64, point.z(), 2.0_f64.sqrt() / 2.0));
        }

        {
            let point = full_quarter * point;
            assert!(approx_eq!(f64, point.x(), 1.0));
            assert_eq!(point.y(), 0.0);
            assert!(approx_eq!(f64, point.z(), 0.0));
        }
    }

    #[test]
    fn should_be_able_to_rotate_around_the_z_axis() {
        let point = Point3D::new(0.0, 1.0, 0.0);
        let half_quarter = Transform::identity().rotate_z(PI / 4.0);
        let full_quarter = Transform::identity().rotate_z(PI / 2.0);

        {
            let point = half_quarter * point;
            assert!(approx_eq!(f64, point.x(), -(2.0_f64.sqrt() / 2.0)));
            assert!(approx_eq!(f64, point.y(), 2.0_f64.sqrt() / 2.0));
            assert_eq!(point.z(), 0.0);
        }

        {
            let point = full_quarter * point;
            assert!(approx_eq!(f64, point.x(), -1.0));
            assert!(approx_eq!(f64, point.y(), 0.0));
            assert_eq!(point.z(), 0.0);
        }
    }

    #[test]
    fn rotating_a_point_around_an_inverted_rotation_matrix_rotates_in_the_opposite_direction() {
        let point = Point3D::new(0.0, 1.0, 0.0);
        let half_quarter = Transform::identity().rotate_x(PI / 4.0);

        {
            let point = half_quarter.inverse().unwrap() * point;
            assert_eq!(point.x(), 0.0);
            assert!(approx_eq!(f64, point.y(), 2.0_f64.sqrt() / 2.0));
            assert!(approx_eq!(f64, point.z(), -(2.0_f64.sqrt() / 2.0)));
        }
    }

    #[test]
    fn an_x_to_y_shear_moves_x_in_proportion_to_y() {
        let point = Point3D::new(2.0, 3.0, 4.0);
        let shear = Transform::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

        let sheared = shear * point;
        assert_eq!(sheared, Point3D::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn an_x_to_z_shear_moves_x_in_proportion_to_z() {
        let point = Point3D::new(2.0, 3.0, 4.0);
        let shear = Transform::shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);

        let sheared = shear * point;
        assert_eq!(sheared, Point3D::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn a_y_to_x_shear_moves_y_in_proportion_to_x() {
        let point = Point3D::new(2.0, 3.0, 4.0);
        let shear = Transform::shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

        let sheared = shear * point;
        assert_eq!(sheared, Point3D::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn a_y_to_z_shear_moves_y_in_proportion_to_z() {
        let point = Point3D::new(2.0, 3.0, 4.0);
        let shear = Transform::shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        let sheared = shear * point;
        assert_eq!(sheared, Point3D::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn a_z_to_x_shear_moves_z_in_proportion_to_x() {
        let point = Point3D::new(2.0, 3.0, 4.0);
        let shear = Transform::shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);

        let sheared = shear * point;
        assert_eq!(sheared, Point3D::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn a_z_to_y_shear_moves_z_in_proportion_to_y() {
        let point = Point3D::new(2.0, 3.0, 4.0);
        let shear = Transform::shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        let sheared = shear * point;
        assert_eq!(sheared, Point3D::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let point = Point3D::new(1.0, 0.0, 1.0);
        let rotation = Transform::identity().rotate_x(PI / 2.0);
        let scale = Transform::scaling(5.0, 5.0, 5.0);
        let translation = Transform::translation(10.0, 5.0, 7.0);

        let rotated = rotation * point;
        {
            let point = rotated;

            assert!(approx_eq!(f64, point.x(), 1.0));
            assert!(approx_eq!(f64, point.y(), -1.0));
            assert!(approx_eq!(f64, point.z(), 0.0));
        }

        let scaled = scale * rotated;
        {
            let point = scaled;

            assert!(approx_eq!(f64, point.x(), 5.0));
            assert!(approx_eq!(f64, point.y(), -5.0));
            assert!(approx_eq!(
                f64,
                point.z(),
                0.0,
                ulps = 5,
                epsilon = f32::EPSILON as f64
            ));
        }

        let translated = translation * scaled;
        {
            let point = translated;

            assert!(approx_eq!(f64, point.x(), 15.0));
            assert!(approx_eq!(f64, point.y(), 0.0));
            assert!(approx_eq!(f64, point.z(), 7.0));
        }
    }

    #[test]
    fn combined_transformations_are_applied_in_reverse_order() {
        let point = Point3D::new(1.0, 0.0, 1.0);
        let rotation = Transform::identity().rotate_x(PI / 2.0);
        let scale = Transform::scaling(5.0, 5.0, 5.0);
        let translation = Transform::translation(10.0, 5.0, 7.0);

        let transform = translation * scale * rotation;
        let point = transform * point;

        assert_eq!(point.x(), 15.0);
        assert_eq!(point.y(), 0.0);
        assert_eq!(point.z(), 7.0);
    }

    #[test]
    fn fluent_api_applies_transformations_in_sequence() {
        let point = Point3D::new(1.0, 0.0, 1.0);

        let translation = Transform::identity()
            .rotate_x(PI / 2.0)
            .scale_all(5.0)
            .translate_x(10.0)
            .translate_y(5.0)
            .translate_z(7.0);

        assert_eq!(translation * point, Point3D::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn the_view_transform_of_the_default_orientation_is_the_identity_matrix() {
        let transform = Transform::view_transform(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.0, 0.0, -1.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );

        assert_eq!(transform, Transform::identity());
    }

    #[test]
    fn the_view_transform_for_a_positive_z_orientation_is_a_scaling_matrix() {
        let transform = Transform::view_transform(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );

        assert_eq!(transform, Transform::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn a_view_translation_moves_the_world_in_the_opposite_direction() {
        let transform = Transform::view_transform(
            Point3D::new(0.0, 0.0, 8.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );

        assert_eq!(transform, Transform::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn an_arbitrary_orientation_produces_the_correct_view_transform() {
        let transform = Transform::view_transform(
            Point3D::new(1.0, 3.0, 2.0),
            Point3D::new(4.0, -2.0, 8.0),
            Vector3D::new(1.0, 1.0, 0.0),
        );

        #[rustfmt::skip]
        assert_eq!(
            transform,
            Transform::new(Matrix4D::new(
                [-0.5070925528371099, 0.5070925528371099, 0.6761234037828132, -2.366431913239846],
                [0.7677159338596801, 0.6060915267313263, 0.12121830534626524, -2.8284271247461894],
                [-0.35856858280031806, 0.5976143046671968, -0.7171371656006361, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ))
        );
    }
}

// TODO write property tests for:
//   - building and combining Transforms using all constructors/builders and ensuring the inverse is always defined
//   - building Transforms and ensure that for all behaviours (inverting, tranposing, multiplying, etc) they behave the same as a Matrix4D
//   - combining Transforms and ensuring the above behaviours are equivalent for combined Matrix4Ds
//   to ensure underlying "optimised" representation is also logically correct
mod property_tests {
    extern crate float_cmp;
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn multiplying_a_vector_by_identity_matrix_produces_the_same_vector(vector in any::<Vector3D>()) {
            assert_eq!(Transform::identity() * vector, vector);
        }

        #[test]
        fn multiplying_a_point_by_identity_matrix_produces_the_same_point(point in any::<Point3D>()) {
            assert_eq!(Transform::identity() * point, point);
        }

        #[test]
        fn multiplying_a_matrix_by_a_matrix_inverse_undoes_multiplication(
            first in any::<Transform>(),
            second in any::<Transform>(),
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

            if second.inverse.is_some() {
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

        #[test]
        fn vectors_cannot_be_translated(vector in any::<Vector3D>(), x in any::<f64>(), y in any::<f64>(), z in any::<f64>()) {
            let translation = Transform::translation(x, y, z);

            assert_eq!(translation * vector, vector);
        }

        #[test]
        fn fluent_translate_api_behaves_the_same_as_translation_matrix(
            point in any::<Point3D>(),
            x in any::<f64>(),
            y in any::<f64>(),
            z in any::<f64>(),
        ) {
            let direct = Transform::translation(x, y, z);
            let fluent = Transform::identity()
                .translate_x(x)
                .translate_y(y)
                .translate_z(z);

            assert_eq!(direct * point, fluent * point);
        }

        #[test]
        fn fluent_scaling_api_behaves_the_same_as_scaling_matrix(
            point in any::<Point3D>(),
            x in any::<f64>(),
            y in any::<f64>(),
            z in any::<f64>(),
        ) {
            let direct = Transform::scaling(x, y, z);
            let fluent = Transform::identity().scale_x(x).scale_y(y).scale_z(z);

            assert_eq!(direct * point, fluent * point);
        }
    }
}
