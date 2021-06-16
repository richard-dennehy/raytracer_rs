use super::*;

mod unit_tests {
    use super::*;
    use approx::*;
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

        let (x, y, z, _) = translation.inverse() * point;
        let translated = Point3D::new(x, y, z);
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

        let (x, y, z, _) = scale.inverse() * vector;
        let scaled = Vector3D::new(x, y, z);
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
            assert_abs_diff_eq!(point.y(), 2.0_f64.sqrt() / 2.0);
            assert_abs_diff_eq!(point.z(), 2.0_f64.sqrt() / 2.0);
        }

        {
            let point = full_quarter * point;
            assert_eq!(point.x(), 0.0);
            assert_abs_diff_eq!(point.y(), 0.0);
            assert_abs_diff_eq!(point.z(), 1.0);
        }
    }

    #[test]
    fn should_be_able_to_rotate_around_the_y_axis() {
        let point = Point3D::new(0.0, 0.0, 1.0);
        let half_quarter = Transform::identity().rotate_y(PI / 4.0);
        let full_quarter = Transform::identity().rotate_y(PI / 2.0);

        {
            let point = half_quarter * point;
            assert_abs_diff_eq!(point.x(), 2.0_f64.sqrt() / 2.0);
            assert_eq!(point.y(), 0.0);
            assert_abs_diff_eq!(point.z(), 2.0_f64.sqrt() / 2.0);
        }

        {
            let point = full_quarter * point;
            assert_abs_diff_eq!(point.x(), 1.0);
            assert_eq!(point.y(), 0.0);
            assert_abs_diff_eq!(point.z(), 0.0);
        }
    }

    #[test]
    fn should_be_able_to_rotate_around_the_z_axis() {
        let point = Point3D::new(0.0, 1.0, 0.0);
        let half_quarter = Transform::identity().rotate_z(PI / 4.0);
        let full_quarter = Transform::identity().rotate_z(PI / 2.0);

        {
            let point = half_quarter * point;
            assert_abs_diff_eq!(point.x(), -(2.0_f64.sqrt() / 2.0));
            assert_abs_diff_eq!(point.y(), 2.0_f64.sqrt() / 2.0);
            assert_eq!(point.z(), 0.0);
        }

        {
            let point = full_quarter * point;
            assert_abs_diff_eq!(point.x(), -1.0);
            assert_abs_diff_eq!(point.y(), 0.0);
            assert_eq!(point.z(), 0.0);
        }
    }

    #[test]
    fn rotating_a_point_around_an_inverted_rotation_matrix_rotates_in_the_opposite_direction() {
        let point = Point3D::new(0.0, 1.0, 0.0);
        let half_quarter = Transform::identity().rotate_x(PI / 4.0);

        {
            let (x, y, z, _) = half_quarter.inverse() * point;

            assert_eq!(x, 0.0);
            assert_abs_diff_eq!(y, 2.0_f64.sqrt() / 2.0);
            assert_abs_diff_eq!(z, -(2.0_f64.sqrt() / 2.0));
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
        assert_abs_diff_eq!(rotated, Point3D::new(1.0, -1.0, 0.0));

        let scaled = scale * rotated;
        assert_abs_diff_eq!(scaled, Point3D::new(5.0, -5.0, 0.0));

        let translated = translation * scaled;
        assert_abs_diff_eq!(translated, Point3D::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn combined_transformations_are_applied_in_reverse_order() {
        let point = Point3D::new(1.0, 0.0, 1.0);
        let rotation = Transform::identity().rotate_x(PI / 2.0);
        let scale = Transform::scaling(5.0, 5.0, 5.0);
        let translation = Transform::translation(10.0, 5.0, 7.0);

        let transform = translation * scale * rotation;
        let point = transform * point;

        assert_abs_diff_eq!(point, Point3D::new(15.0, 0.0, 7.0));
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

        assert_abs_diff_eq!(translation * point, Point3D::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn the_view_transform_of_the_default_orientation_is_the_identity_matrix() {
        let transform = Transform::view_transform(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.0, 0.0, -1.0),
            Normal3D::POSITIVE_Y,
        );

        assert_eq!(transform, Transform::identity());
    }

    #[test]
    fn the_view_transform_for_a_positive_z_orientation_is_a_scaling_matrix() {
        let transform = Transform::view_transform(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.0, 0.0, 1.0),
            Normal3D::POSITIVE_Y,
        );

        assert_eq!(transform, Transform::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn a_view_translation_moves_the_world_in_the_opposite_direction() {
        let transform = Transform::view_transform(
            Point3D::new(0.0, 0.0, 8.0),
            Point3D::new(0.0, 0.0, 0.0),
            Normal3D::POSITIVE_Y,
        );

        assert_eq!(transform, Transform::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn an_arbitrary_orientation_produces_the_correct_view_transform() {
        let transform = Transform::view_transform(
            Point3D::new(1.0, 3.0, 2.0),
            Point3D::new(4.0, -2.0, 8.0),
            Vector3D::new(1.0, 1.0, 0.0).normalised(),
        );

        assert_abs_diff_eq!(
            transform,
            Transform::new(Matrix4D::new(
                [
                    -0.5070925528371099,
                    0.5070925528371099,
                    0.6761234037828132,
                    -2.366431913239846
                ],
                [
                    0.7677159338596801,
                    0.6060915267313263,
                    0.12121830534626524,
                    -2.8284271247461894
                ],
                [
                    -0.35856858280031806,
                    0.5976143046671968,
                    -0.7171371656006361,
                    0.0
                ],
                [0.0, 0.0, 0.0, 1.0]
            ))
        );
    }
}

mod property_tests {
    use super::*;
    use proptest::prelude::*;

    // because the IDE totally gives up in the `proptest` macro, define functions here and
    // call them inside the `proptest` block
    mod properties {
        use super::*;

        pub fn identity_transform_multiplication_has_no_effect_on_vector(vector: Vector3D) {
            assert_eq!(Transform::identity() * vector, vector);
        }

        pub fn identity_transform_multiplication_has_no_effect_on_point(point: Point3D) {
            assert_eq!(Transform::identity() * point, point);
        }

        pub fn vectors_cannot_be_translated(vector: Vector3D, translation: Transform) {
            assert_eq!(translation * vector, vector);
        }

        pub fn must_be_invertible(transform: Transform) {
            // if the underlying matrix is not invertible, the transform will panic on creation - this test only ensures that the internal invariant is properly maintained
            assert!(transform.inverse.inverse().is_some())
        }

        pub fn multiplying_a_point_by_a_transform_produces_the_same_point_as_multiplying_by_the_equivalent_matrix(
            transform: Transform,
            point: Point3D,
        ) {
            let expected = transform * point;
            let (x, y, z, _) = transform.underlying() * point;
            let actual = Point3D::new(x, y, z);

            assert_eq!(expected, actual);
        }

        pub fn multiplying_a_vector_by_a_transform_produces_the_same_vector_as_multiplying_by_the_equivalent_matrix(
            transform: Transform,
            vector: Vector3D,
        ) {
            let expected = transform * vector;
            let (x, y, z, _) = transform.underlying() * vector;
            let actual = Vector3D::new(x, y, z);

            assert_eq!(expected, actual);
        }
    }

    proptest! {
        #[test]
        fn multiplying_a_vector_by_identity_transform_produces_the_same_vector(vector in any::<Vector3D>()) {
            properties::identity_transform_multiplication_has_no_effect_on_vector(vector);
        }

        #[test]
        fn multiplying_a_point_by_identity_transform_produces_the_same_point(point in any::<Point3D>()) {
            properties::identity_transform_multiplication_has_no_effect_on_point(point);
        }

        #[test]
        fn vectors_cannot_be_translated(vector in any::<Vector3D>(), translation in Transform::any_translation()) {
            properties::vectors_cannot_be_translated(vector, translation);
        }

        #[test]
        fn all_translations_are_invertible(translation in Transform::any_translation()) {
            properties::must_be_invertible(translation);
        }

        #[test]
        fn all_scaling_is_invertible(scale in Transform::any_scaling()) {
            properties::must_be_invertible(scale);
        }

        #[test]
        fn all_shearing_is_invertible(shear in Transform::any_shear()) {
            properties::must_be_invertible(shear);
        }

        #[test]
        fn all_rotations_are_invertible(rotation in Transform::any_rotation()) {
            properties::must_be_invertible(rotation);
        }

        #[test]
        fn all_transformations_are_invertible(transform in Transform::any_transform()) {
            properties::must_be_invertible(transform);
        }

        #[test]
        fn multiplying_a_point_by_a_transform_produces_the_same_point_as_multiplying_by_the_equivalent_matrix(
            transform in Transform::any_transform(),
            point in any::<Point3D>()
        ) {
            properties::multiplying_a_point_by_a_transform_produces_the_same_point_as_multiplying_by_the_equivalent_matrix(transform, point)
        }

        #[test]
        fn multiplying_a_vector_by_a_transform_produces_the_same_vector_as_multiplying_by_the_equivalent_matrix(
            transform in Transform::any_transform(),
            vector in any::<Vector3D>()
        ) {
            properties::multiplying_a_vector_by_a_transform_produces_the_same_vector_as_multiplying_by_the_equivalent_matrix(transform, vector)
        }
    }
}
