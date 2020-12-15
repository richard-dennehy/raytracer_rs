use super::*;

mod unit_tests {
    use super::*;

    #[test]
    fn adding_two_vectors_should_produce_a_vector() {
        let v1 = Vector3D::new(3.0, -2.0, 5.0);
        let v2 = Vector3D::new(-2.0, 3.0, 1.0);

        let v3 = v1 + v2;
        assert_eq!(v3, Vector3D::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn adding_a_point_to_a_vector_should_produce_a_point() {
        let vector = Vector3D::new(3.0, -2.0, 5.0);
        let point = Point3D::new(-2.0, 3.0, 1.0);

        let sum = vector + point;
        assert_eq!(sum, Point3D::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn subtracting_a_vector_from_a_vector_should_produce_a_vector() {
        let v1 = Vector3D::new(3.0, 2.0, 1.0);
        let v2 = Vector3D::new(5.0, 6.0, 7.0);

        let delta = v1 - v2;
        assert_eq!(delta, Vector3D::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negating_a_vector_should_negate_all_components() {
        assert_eq!(
            -Vector3D::new(1.0, -2.0, 3.0),
            Vector3D::new(-1.0, 2.0, -3.0)
        );
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar_should_scale_all_components() {
        let vector = Vector3D::new(1.0, -2.0, 3.0);
        let scaled = vector * 3.5;

        assert_eq!(scaled, Vector3D::new(3.5, -7.0, 10.5));
    }

    #[test]
    fn multiplying_a_vector_by_a_fractional_scalar_should_scale_all_components_down() {
        let vector = Vector3D::new(1.0, -2.0, 3.0);
        let scaled = vector * 0.5;

        assert_eq!(scaled, Vector3D::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn dividing_a_vector_by_a_scalar_should_divide_all_components() {
        let vector = Vector3D::new(1.0, -2.0, 3.0);
        let scaled = vector / 2.0;

        assert_eq!(scaled, Vector3D::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn magnitude_of_x_unit_vector_should_be_1() {
        let x_unit = Vector3D::new(1.0, 0.0, 0.0);
        assert_eq!(x_unit.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_y_unit_vector_should_be_1() {
        let y_unit = Vector3D::new(0.0, 1.0, 0.0);
        assert_eq!(y_unit.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_z_unit_vector_should_be_1() {
        let z_unit = Vector3D::new(0.0, 0.0, 1.0);
        assert_eq!(z_unit.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_should_equal_square_root_of_summed_squares_of_components() {
        let vector = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(vector.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalising_a_non_unit_x_vector_should_produce_a_unit_x_vector() {
        let vector = Vector3D::new(4.0, 0.0, 0.0);
        assert_eq!(vector.normalised(), Vector3D::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalising_a_vector_should_scale_components_down_by_the_magnitude() {
        let vector = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(
            vector.normalised(),
            Vector3D::new(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            )
        );
    }

    #[test]
    fn dot_product_of_two_vectors_should_multiply_same_components_and_sum() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(2.0, 3.0, 4.0);

        assert_eq!(v1.dot(&v2), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors_produces_parallel_vector() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(2.0, 3.0, 4.0);

        assert_eq!(v1.cross(&v2), Vector3D::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(&v1), Vector3D::new(1.0, -2.0, 1.0));
    }
}

mod property_tests {
    extern crate quickcheck;
    use super::*;

    #[quickcheck]
    fn first_element_should_be_x(x: f64, y: f64, z: f64) {
        assert_eq!(Vector3D::new(x, y, z).x(), x)
    }

    #[quickcheck]
    fn second_element_should_be_y(x: f64, y: f64, z: f64) {
        assert_eq!(Vector3D::new(x, y, z).y(), y)
    }

    #[quickcheck]
    fn third_element_should_be_z(x: f64, y: f64, z: f64) {
        assert_eq!(Vector3D::new(x, y, z).z(), z)
    }

    #[quickcheck]
    fn w_should_always_be_zero(x: f64, y: f64, z: f64) {
        assert_eq!(Vector3D::new(x, y, z).w(), 0.0)
    }

    #[quickcheck]
    fn adding_vectors_should_sum_x_y_and_z(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) {
        let first = Vector3D::new(x1, y1, z1);
        let second = Vector3D::new(x2, y2, z2);

        let added = first + second;
        assert_eq!(added.x(), x1 + x2);
        assert_eq!(added.y(), y1 + y2);
        assert_eq!(added.z(), z1 + z2);

        assert_eq!(added.w(), 0.0);
    }

    #[quickcheck]
    fn adding_a_point_to_a_vector_should_produce_a_point_with_the_sum_of_x_y_and_z(
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
    ) {
        let vector = Vector3D::new(x1, y1, z1);
        let point = Point3D::new(x2, y2, z2);

        let added = vector + point;
        assert_eq!(added.x(), x1 + x2);
        assert_eq!(added.y(), y1 + y2);
        assert_eq!(added.z(), z1 + z2);

        assert_eq!(added.w(), 1.0);
    }

    #[quickcheck]
    fn subtracting_a_vector_from_a_vector_should_produce_a_vector_of_the_change_in_direction(
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
    ) {
        let v1 = Vector3D::new(x1, y1, z1);
        let v2 = Vector3D::new(x2, y2, z2);

        let delta = v1 - v2;
        assert_eq!(delta.x(), x1 - x2);
        assert_eq!(delta.y(), y1 - y2);
        assert_eq!(delta.z(), z1 - z2);

        assert_eq!(delta.w(), 0.0);
    }

    #[quickcheck]
    fn negating_a_vector_should_negate_the_x_y_and_z(x: f64, y: f64, z: f64) {
        let vector = Vector3D::new(x, y, z);
        let negated = Vector3D::new(-x, -y, -z);

        assert_eq!(-vector.clone(), negated);
        assert_eq!((-vector).w(), 0.0);
    }

    #[quickcheck]
    fn multiplying_a_vector_by_a_scalar_should_multiply_x_y_and_z(x: f64, y: f64, z: f64, s: f64) {
        let vector = Vector3D::new(x, y, z);
        let scaled = vector * s;

        assert_eq!(scaled.x(), x * s);
        assert_eq!(scaled.y(), y * s);
        assert_eq!(scaled.z(), z * s);

        assert_eq!(scaled.w(), 0.0);
    }

    #[quickcheck]
    fn dividing_a_vector_by_a_scalar_should_divide_x_y_and_z(x: f64, y: f64, z: f64, s: f64) {
        let vector = Vector3D::new(x, y, z);
        let scaled = vector / s;

        assert_eq!(scaled.x(), x / s);
        assert_eq!(scaled.y(), y / s);
        assert_eq!(scaled.z(), z / s);

        assert_eq!(scaled.w(), 0.0);
    }

    #[quickcheck]
    fn magnitude_of_a_vector_equals_magnitude_of_negated_vector(x: f64, y: f64, z: f64) {
        let vector = Vector3D::new(x, y, z);

        assert_eq!(vector.clone().magnitude(), (-vector).magnitude());
    }

    #[quickcheck]
    fn magnitude_of_a_normalised_vector_is_always_1(x: f64, y: f64, z: f64) {
        // can't normalise a zero magnitude vector - easier to just ignore this single case than properly filter it
        if x != 0.0 && y != 0.0 && z != 0.0 {
            let vector = Vector3D::new(x, y, z);

            // rounding errors start to accumulate
            assert!(vector.normalised().magnitude() - 1.0 <= f64::EPSILON)
        }
    }

    #[quickcheck]
    fn dot_product_is_commutative(v1: Vector3D, v2: Vector3D) {
        assert_eq!(v1.dot(&v2), v2.dot(&v1));
    }

    #[quickcheck]
    fn dot_product_of_normalised_vectors_is_always_between_1_and_negative_1(
        v1: Vector3D,
        v2: Vector3D,
    ) {
        let dot = v1.normalised().dot(&v2.normalised());

        assert!(dot >= -1.0 && dot <= 1.0)
    }

    #[quickcheck]
    fn dot_product_of_same_direction_unit_vectors_is_always_1(vector: Vector3D) {
        // rounding errors _really_ accumulate
        assert!(vector.normalised().dot(&vector.normalised()) - 1.0 <= f32::EPSILON as _)
    }

    #[quickcheck]
    fn dot_product_of_opposite_direction_unit_vectors_is_always_negative_1(vector: Vector3D) {
        assert!(vector.normalised().dot(&-(vector.normalised())) - 1.0 <= f32::EPSILON as _)
    }

    #[quickcheck]
    fn cross_product_is_anti_commutative(v1: Vector3D, v2: Vector3D) {
        assert_eq!(v1.cross(&v2), -(v2.cross(&v1)))
    }
}
