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
}

mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn first_element_should_be_x(x in crate::util::reasonable_f64(), y in crate::util::reasonable_f64(), z in crate::util::reasonable_f64()) {
            assert_eq!(Point3D::new(x, y, z).x(), x)
        }

        #[test]
        fn second_element_should_be_y(x in crate::util::reasonable_f64(), y in crate::util::reasonable_f64(), z in crate::util::reasonable_f64()) {
            assert_eq!(Point3D::new(x, y, z).y(), y)
        }

        #[test]
        fn third_element_should_be_z(x in crate::util::reasonable_f64(), y in crate::util::reasonable_f64(), z in crate::util::reasonable_f64()) {
            assert_eq!(Point3D::new(x, y, z).z(), z)
        }

        #[test]
        fn w_should_always_be_one(x in crate::util::reasonable_f64(), y in crate::util::reasonable_f64(), z in crate::util::reasonable_f64()) {
            assert_eq!(Point3D::new(x, y, z).w(), 1.0)
        }

        #[test]
        fn adding_a_vector_to_a_point_should_produce_a_point_with_sum_of_x_y_and_z(
            x1 in crate::util::reasonable_f64(),
            y1 in crate::util::reasonable_f64(),
            z1 in crate::util::reasonable_f64(),
            x2 in crate::util::reasonable_f64(),
            y2 in crate::util::reasonable_f64(),
            z2 in crate::util::reasonable_f64(),
        ) {
            let point = Point3D::new(x1, y1, z1);
            let vector = Vector3D::new(x2, y2, z2);

            let added = point + vector;
            assert_eq!(added.x(), x1 + x2);
            assert_eq!(added.y(), y1 + y2);
            assert_eq!(added.z(), z1 + z2);

            assert_eq!(added.w(), 1.0);
        }

        #[test]
        fn subtracting_a_point_from_a_point_should_produce_a_vector_of_the_distance_between_them(
            x1 in crate::util::reasonable_f64(),
            y1 in crate::util::reasonable_f64(),
            z1 in crate::util::reasonable_f64(),
            x2 in crate::util::reasonable_f64(),
            y2 in crate::util::reasonable_f64(),
            z2 in crate::util::reasonable_f64(),
        ) {
            let p1 = Point3D::new(x1, y1, z1);
            let p2 = Point3D::new(x2, y2, z2);

            let distance = p1 - p2;
            assert_eq!(distance.x(), x1 - x2);
            assert_eq!(distance.y(), y1 - y2);
            assert_eq!(distance.z(), z1 - z2);

            assert_eq!(distance.w(), 0.0);
        }

        #[test]
        fn subtracting_a_vector_from_a_point_should_produce_a_point_translated_by_the_vector(
            x1 in crate::util::reasonable_f64(),
            y1 in crate::util::reasonable_f64(),
            z1 in crate::util::reasonable_f64(),
            x2 in crate::util::reasonable_f64(),
            y2 in crate::util::reasonable_f64(),
            z2 in crate::util::reasonable_f64(),
        ) {
            let point = Point3D::new(x1, y1, z1);
            let vector = Vector3D::new(x2, y2, z2);

            let translated = point - vector;
            assert_eq!(translated.x(), x1 - x2);
            assert_eq!(translated.y(), y1 - y2);
            assert_eq!(translated.z(), z1 - z2);

            assert_eq!(translated.w(), 1.0);
        }
    }
}
