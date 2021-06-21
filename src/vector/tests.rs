use super::*;

mod unit_tests {
    use super::*;
    use approx::*;

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
        assert_eq!(vector.normalised(), Normal3D::POSITIVE_X);
    }

    #[test]
    fn normalising_a_vector_should_scale_components_down_by_the_magnitude() {
        let vector = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(
            vector.normalised(),
            Normal3D::new(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            )
        );
    }

    /// this isn't mathematically correct, but given the options of:
    ///  - return a Vector with `NaN` components (division by 0)
    ///  - return another zero vector (which means `normalised` doesn't always return a unit vector)
    ///  - return `None` for zero vectors (and make the return type optional)
    ///  - returning an arbitrary unit vector
    ///  - panicking
    /// returning a zero vector seems "least worst" for the time being
    #[test]
    fn normalising_a_zero_vector_should_produce_a_zero_vector() {
        let vector = Vector3D::new(0.0, 0.0, 0.0);

        assert_eq!(vector.normalised(), Normal3D::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn dot_product_of_two_vectors_should_multiply_same_components_and_sum() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(2.0, 3.0, 4.0);

        assert_eq!(v1.dot(v2), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors_produces_parallel_vector() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(2.0, 3.0, 4.0);

        assert_eq!(v1.cross(v2), Vector3D::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(v1), Vector3D::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn should_correctly_reflect_a_vector_at_a_45_degree_angle_to_the_normal_plane() {
        let vector = Vector3D::new(1.0, -1.0, 0.0);
        let normal = Normal3D::POSITIVE_Y;

        let reflected = vector.reflect_through(normal);
        assert_eq!(reflected, Vector3D::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn should_correctly_reflect_a_vector_off_a_slanted_plane() {
        let vector = Vector3D::new(0.0, -1.0, 0.0);
        let normal = Normal3D::new(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);

        let reflected = vector.reflect_through(normal);
        assert_abs_diff_eq!(reflected, Vector3D::new(1.0, 0.0, 0.0));
    }
}

mod property_tests {
    use super::*;
    use crate::util::{F64Ext, ReasonableF64};
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn adding_vectors_should_produce_a_vector_over_the_combined_distance(
        first: Vector3D,
        second: Vector3D,
    ) {
        let added = first + second;
        assert_eq!(added.x(), first.x() + second.x());
        assert_eq!(added.y(), first.y() + second.y());
        assert_eq!(added.z(), first.z() + second.z());
    }

    #[quickcheck]
    fn adding_a_point_to_a_vector_should_produce_a_point_translated_by_the_vector(
        vector: Vector3D,
        point: Point3D,
    ) {
        let added = vector + point;
        assert_eq!(added.x(), vector.x() + point.x());
        assert_eq!(added.y(), vector.y() + point.y());
        assert_eq!(added.z(), vector.z() + point.z());
    }

    #[quickcheck]
    fn subtracting_a_vector_from_a_vector_should_produce_a_vector_of_the_change_in_direction(
        v1: Vector3D,
        v2: Vector3D,
    ) {
        let delta = v1 - v2;
        assert_eq!(delta.x(), v1.x() - v2.x());
        assert_eq!(delta.y(), v1.y() - v2.y());
        assert_eq!(delta.z(), v1.z() - v2.z());
    }

    #[quickcheck]
    fn negating_a_vector_should_negate_the_x_y_and_z(vector: Vector3D) {
        let negated = Vector3D::new(-vector.x(), -vector.y(), -vector.z());

        assert_eq!(-vector.clone(), negated);
    }

    #[quickcheck]
    fn multiplying_a_vector_by_a_scalar_should_multiply_x_y_and_z(
        vector: Vector3D,
        s: ReasonableF64,
    ) {
        let s = s.0;
        let scaled = vector * s;

        assert_eq!(scaled.x(), vector.x() * s);
        assert_eq!(scaled.y(), vector.y() * s);
        assert_eq!(scaled.z(), vector.z() * s);
    }

    #[quickcheck]
    fn dividing_a_vector_by_a_scalar_should_divide_x_y_and_z(vector: Vector3D, s: ReasonableF64) {
        let s = s.0;
        let scaled = vector / s;

        assert_eq!(scaled.x(), vector.x() / s);
        assert_eq!(scaled.y(), vector.y() / s);
        assert_eq!(scaled.z(), vector.z() / s);
    }

    #[quickcheck]
    fn negating_a_vector_preserves_the_magnitude(vector: Vector3D) {
        assert_eq!(vector.clone().magnitude(), (-vector).magnitude());
    }

    #[quickcheck]
    fn magnitude_of_a_normalised_vector_is_always_1(
        x: ReasonableF64,
        y: ReasonableF64,
        z: ReasonableF64,
    ) {
        let (x, y, z) = (x.0, y.0, z.0);
        // can't normalise a zero magnitude vector - easier to just ignore this single case than properly filter it
        if x != 0.0 && y != 0.0 && z != 0.0 {
            let vector = Vector3D::new(x, y, z);

            assert!(vector.normalised().magnitude().roughly_equals(1.0))
        }
    }

    #[quickcheck]
    fn dot_product_is_commutative(v1: Vector3D, v2: Vector3D) {
        assert_eq!(v1.dot(v2), v2.dot(v1));
    }

    #[quickcheck]
    fn dot_product_of_normalised_vectors_is_always_between_1_and_negative_1(
        v1: Vector3D,
        v2: Vector3D,
    ) {
        let dot = v1.normalised().dot(v2.normalised());

        assert!(dot >= -1.0 && dot <= 1.0)
    }

    #[quickcheck]
    fn dot_product_of_same_direction_unit_vectors_is_always_1(vector: Vector3D) {
        assert!(vector
            .normalised()
            .dot(vector.normalised())
            .roughly_equals(1.0))
    }

    #[quickcheck]
    fn dot_product_of_opposite_direction_unit_vectors_is_always_negative_1(vector: Vector3D) {
        assert!(vector
            .normalised()
            .dot(-(vector.normalised()))
            .roughly_equals(-1.0))
    }

    #[quickcheck]
    fn cross_product_is_anti_commutative(v1: Vector3D, v2: Vector3D) {
        assert_eq!(v1.cross(v2), -(v2.cross(v1)))
    }
}
