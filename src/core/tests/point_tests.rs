use super::*;

mod unit_tests {
    use super::*;

    #[test]
    fn adding_a_vector_to_a_point_should_produce_a_point() {
        let point = Point3D::new(-2.0, 3.0, 1.0);
        let vector = Vector3D::new(3.0, -2.0, 5.0);

        let sum = point + vector;
        assert_eq!(sum, Point3D::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn subtracting_a_point_from_a_point_should_produce_a_vector() {
        let p1 = Point3D::new(3.0, 2.0, 1.0);
        let p2 = Point3D::new(5.0, 6.0, 7.0);

        let vector = p1 - p2;
        assert_eq!(vector, Vector3D::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_a_point_should_produce_a_point() {
        let point = Point3D::new(3.0, 2.0, 1.0);
        let vector = Vector3D::new(5.0, 6.0, 7.0);

        let translated = point - vector;
        assert_eq!(translated, Point3D::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn the_min_of_an_array_of_points_should_lie_at_the_minimum_x_y_and_z_positions() {
        let min = Point3D::min([
            Point3D::new(0.0, 1.0, -2.0),
            Point3D::new(-1.0, 2.0, 3.0),
            Point3D::new(2.0, -3.0, 4.0),
        ]);

        assert_eq!(min, Point3D::new(-1.0, -3.0, -2.0));
    }

    #[test]
    fn the_max_of_an_array_of_points_should_lie_at_the_maximum_x_y_and_z_positions() {
        let max = Point3D::max([
            Point3D::new(0.0, 1.0, -2.0),
            Point3D::new(-1.0, 2.0, 3.0),
            Point3D::new(2.0, -3.0, 4.0),
        ]);

        assert_eq!(max, Point3D::new(2.0, 2.0, 4.0));
    }
}

mod property_tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn adding_a_vector_to_a_point_should_produce_a_point_translated_by_the_vector(
        point: Point3D,
        vector: Vector3D,
    ) {
        let translated = point + vector;
        assert_eq!(translated.x(), point.x() + vector.x());
        assert_eq!(translated.y(), point.y() + vector.y());
        assert_eq!(translated.z(), point.z() + vector.z());
    }

    #[quickcheck]
    fn subtracting_a_point_from_a_point_should_produce_a_vector_of_the_distance_between_them(
        p1: Point3D,
        p2: Point3D,
    ) {
        let distance = p1 - p2;
        assert_eq!(distance.x(), p1.x() - p2.x());
        assert_eq!(distance.y(), p1.y() - p2.y());
        assert_eq!(distance.z(), p1.z() - p2.z());
    }

    #[quickcheck]
    fn subtracting_a_vector_from_a_point_should_produce_a_point_translated_by_the_negative_vector(
        point: Point3D,
        vector: Vector3D,
    ) {
        let translated = point - vector;
        assert_eq!(translated.x(), point.x() - vector.x());
        assert_eq!(translated.y(), point.y() - vector.y());
        assert_eq!(translated.z(), point.z() - vector.z());
    }
}
