use super::*;

mod unit_tests {
    use super::*;
    use std::f64::consts::{PI, SQRT_2};

    #[test]
    fn expanding_a_bounding_box_to_contain_a_smaller_box_should_return_the_outer_box() {
        let outer = BoundingBox::new(Point3D::new(-2.0, -2.0, -2.0), Point3D::new(2.0, 2.0, 2.0));
        let inner = BoundingBox::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0));

        let expanded = outer.expand_to_fit(&inner);
        assert_eq!(expanded.min, Point3D::new(-2.0, -2.0, -2.0));
        assert_eq!(expanded.max, Point3D::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn expanding_a_bounding_box_to_contain_a_wider_box_should_make_the_outer_box_wider() {
        let outer = BoundingBox::new(Point3D::new(-2.0, -2.0, -2.0), Point3D::new(2.0, 2.0, 2.0));
        let inner = BoundingBox::new(Point3D::new(-3.0, -1.0, -1.0), Point3D::new(3.0, 1.0, 1.0));

        let expanded = outer.expand_to_fit(&inner);
        assert_eq!(expanded.min, Point3D::new(-3.0, -2.0, -2.0));
        assert_eq!(expanded.max, Point3D::new(3.0, 2.0, 2.0));
    }

    #[test]
    fn expanding_a_bounding_box_to_contain_a_taller_box_should_make_the_outer_box_taller() {
        let outer = BoundingBox::new(Point3D::new(-2.0, -2.0, -2.0), Point3D::new(2.0, 2.0, 2.0));
        let inner = BoundingBox::new(Point3D::new(-1.0, -3.0, -1.0), Point3D::new(1.0, 3.0, 1.0));

        let expanded = outer.expand_to_fit(&inner);
        assert_eq!(expanded.min, Point3D::new(-2.0, -3.0, -2.0));
        assert_eq!(expanded.max, Point3D::new(2.0, 3.0, 2.0));
    }

    #[test]
    fn expanding_a_bounding_box_to_contain_a_deeper_box_should_make_the_outer_box_deeper() {
        let outer = BoundingBox::new(Point3D::new(-2.0, -2.0, -2.0), Point3D::new(2.0, 2.0, 2.0));
        let inner = BoundingBox::new(Point3D::new(-1.0, -1.0, -3.0), Point3D::new(1.0, 1.0, 3.0));

        let expanded = outer.expand_to_fit(&inner);
        assert_eq!(expanded.min, Point3D::new(-2.0, -2.0, -3.0));
        assert_eq!(expanded.max, Point3D::new(2.0, 2.0, 3.0));
    }

    #[test]
    fn expanding_a_bounding_box_to_contain_a_larger_box_should_return_the_larger_box() {
        let outer = BoundingBox::new(Point3D::new(-2.0, -2.0, -2.0), Point3D::new(2.0, 2.0, 2.0));
        let inner = BoundingBox::new(Point3D::new(-3.0, -3.0, -3.0), Point3D::new(3.0, 3.0, 3.0));

        let expanded = outer.expand_to_fit(&inner);
        assert_eq!(expanded.min, Point3D::new(-3.0, -3.0, -3.0));
        assert_eq!(expanded.max, Point3D::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn a_bounding_box_should_contain_a_point_within_the_bounds() {
        let bounds = BoundingBox::new(Point3D::new(5.0, -2.0, 0.0), Point3D::new(11.0, 4.0, 7.0));

        vec![
            ("min point", Point3D::new(5.0, -2.0, 0.0)),
            ("max point", Point3D::new(11.0, 4.0, 7.0)),
            ("centre", Point3D::new(8.0, 1.0, 3.5)),
        ]
        .into_iter()
        .for_each(|(scenario, point)| assert!(bounds.contains(point), "{}", scenario))
    }

    #[test]
    fn a_bounding_box_should_not_contain_a_point_outside_the_bounds() {
        let bounds = BoundingBox::new(Point3D::new(5.0, -2.0, 0.0), Point3D::new(11.0, 4.0, 7.0));

        vec![
            ("less than min x", Point3D::new(3.0, 0.0, 3.0)),
            ("less than min y", Point3D::new(8.0, -4.0, 3.0)),
            ("less than min z", Point3D::new(8.0, 1.0, -1.0)),
            ("greater than max x", Point3D::new(13.0, 1.0, 3.0)),
            ("greater than max y", Point3D::new(8.0, 5.0, 3.0)),
            ("greater than max z", Point3D::new(8.0, 1.0, 8.0)),
        ]
        .into_iter()
        .for_each(|(scenario, point)| assert!(bounds.excludes(point), "{}", scenario))
    }

    #[test]
    fn a_bounding_box_should_contain_another_bounding_box_when_the_other_min_and_max_are_within_the_bounds(
    ) {
        let outer = BoundingBox::new(Point3D::new(5.0, -2.0, 0.0), Point3D::new(11.0, 4.0, 7.0));

        vec![
            (
                "same size",
                BoundingBox::new(Point3D::new(5.0, -2.0, 0.0), Point3D::new(11.0, 4.0, 7.0)),
            ),
            (
                "smaller",
                BoundingBox::new(Point3D::new(6.0, -1.0, 1.0), Point3D::new(10.0, 3.0, 6.0)),
            ),
        ]
        .into_iter()
        .for_each(|(scenario, inner)| assert!(outer.fully_contains(&inner), "{}", scenario))
    }

    #[test]
    fn a_bounding_box_should_exclude_another_bounding_box_when_the_other_min_or_max_are_outside_the_bounds(
    ) {
        let outer = BoundingBox::new(Point3D::new(5.0, -2.0, 0.0), Point3D::new(11.0, 4.0, 7.0));

        vec![
            (
                "min is outside",
                BoundingBox::new(Point3D::new(4.0, -3.0, 1.0), Point3D::new(11.0, 4.0, 7.0)),
            ),
            (
                "max is outside",
                BoundingBox::new(Point3D::new(5.0, -2.0, 0.0), Point3D::new(12.0, 5.0, 8.0)),
            ),
            (
                "min and max are outside",
                BoundingBox::new(Point3D::new(4.0, -3.0, 1.0), Point3D::new(12.0, 5.0, 8.0)),
            ),
        ]
        .into_iter()
        .for_each(|(scenario, inner)| assert!(outer.partially_excludes(&inner), "{}", scenario))
    }

    #[test]
    fn transforming_a_bounding_box_with_a_scaling_matrix_should_scale_it() {
        let bounds = BoundingBox::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0));
        let transform = Transform::identity().scale_all(2.0);

        let scaled = bounds.transformed(transform);
        assert_eq!(scaled.min, Point3D::new(-2.0, -2.0, -2.0));
        assert_eq!(scaled.max, Point3D::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn transforming_a_bounding_box_with_a_translation_matrix_should_translate_it() {
        let bounds = BoundingBox::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0));
        let transform = Transform::identity()
            .translate_x(1.0)
            .translate_y(1.0)
            .translate_z(1.0);

        let scaled = bounds.transformed(transform);
        assert_eq!(scaled.min, Point3D::ORIGIN);
        assert_eq!(scaled.max, Point3D::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn transforming_a_bounding_box_with_a_rotation_matrix_should_scale_the_bounds_to_fit_the_rotated_points(
    ) {
        let bounds = BoundingBox::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0));
        let transform = Transform::identity().rotate_y(PI / 4.0).rotate_x(PI / 4.0);

        let scaled = bounds.transformed(transform);
        assert!(approx_eq!(
            Point3D,
            scaled.min,
            Point3D::new(-SQRT_2, -1.7071067811865475, -1.7071067811865475)
        ));
        assert!(approx_eq!(
            Point3D,
            scaled.max,
            Point3D::new(SQRT_2, 1.7071067811865475, 1.7071067811865475)
        ));
    }
}

mod property_tests {
    use super::*;
    use proptest::prelude::*;

    mod properties {
        use super::*;

        pub fn two_joined_bounding_boxes_should_contain_the_min_and_max_points_of_both_boxes(
            bb1: BoundingBox,
            bb2: BoundingBox,
        ) {
            let bounds = bb1.expand_to_fit(&bb2);

            assert!(bounds.contains(bb1.min));
            assert!(bounds.contains(bb2.min));
            assert!(bounds.contains(bb1.max));
            assert!(bounds.contains(bb2.max));
        }

        pub fn a_bounding_box_should_contain_its_min_and_max_points(bounds: BoundingBox) {
            assert!(bounds.contains(bounds.min));
            assert!(bounds.contains(bounds.max));
        }
    }

    proptest! {
        #[test]
        fn two_joined_bounding_boxes_should_contain_the_min_and_max_points_of_both_boxes(
            bb1 in any::<BoundingBox>(),
            bb2 in any::<BoundingBox>(),
        ) {
            properties::two_joined_bounding_boxes_should_contain_the_min_and_max_points_of_both_boxes(bb1, bb2)
        }

        #[test]
        fn a_bounding_box_should_contain_its_min_and_max_points(
            bounds in any::<BoundingBox>()
        ) {
            properties::a_bounding_box_should_contain_its_min_and_max_points(bounds)
        }
    }
}
