use super::*;

mod unit_tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn a_striped_pattern_uses_the_primary_colour_on_even_x_integer_values() {
        let pattern = Pattern::striped(Colour::WHITE, Colour::BLACK);

        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 1.0, 1.0)),
            Colour::WHITE
        );
    }

    #[test]
    fn a_striped_pattern_uses_the_primary_colour_on_odd_x_integer_values() {
        let pattern = Pattern::striped(Colour::WHITE, Colour::BLACK);

        assert_eq!(
            pattern.colour_at(Point3D::new(1.0, 1.0, 1.0)),
            Colour::BLACK
        );
    }

    #[test]
    fn a_striped_pattern_truncates_positive_x_values_to_check_evenness() {
        let pattern = Pattern::striped(Colour::WHITE, Colour::BLACK);

        assert_eq!(
            pattern.colour_at(Point3D::new(0.9, 1.0, 1.0)),
            Colour::WHITE
        );
    }

    #[test]
    fn a_striped_pattern_rounds_negative_x_values_down_to_check_evenness() {
        let pattern = Pattern::striped(Colour::WHITE, Colour::BLACK);

        assert_eq!(
            pattern.colour_at(Point3D::new(-0.1, 1.0, 1.0)),
            Colour::BLACK
        );
    }

    #[test]
    fn translating_a_striped_pattern_by_x_1_should_invert_the_stripes() {
        let pattern = Pattern::striped(Colour::WHITE, Colour::BLACK)
            .with_transform(Transform::identity().translate_x(1.0));

        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 1.0, 1.0)),
            Colour::BLACK
        );

        assert_eq!(
            pattern.colour_at(Point3D::new(1.0, 1.0, 1.0)),
            Colour::WHITE
        );
    }

    #[test]
    fn rotating_a_striped_pattern_90_deg_around_y_alternates_stripes_along_z() {
        let pattern = Pattern::striped(Colour::WHITE, Colour::BLACK)
            .with_transform(Transform::identity().rotate_y(PI / 2.0));

        assert_eq!(
            pattern.colour_at(Point3D::new(1.0, 0.0, 0.0)),
            Colour::WHITE
        );

        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 0.0, 1.0)),
            Colour::BLACK
        );
    }

    #[test]
    fn shearing_a_striped_pattern_by_z_makes_the_stripes_diagonal_along_xz() {
        let pattern = Pattern::striped(Colour::WHITE, Colour::BLACK)
            .with_transform(Transform::identity().shear_x_to_z(1.0));

        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 0.0, 0.0)),
            Colour::WHITE,
            "Origin"
        );

        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 0.0, 1.0)),
            Colour::BLACK,
            ""
        );

        assert_eq!(
            pattern.colour_at(Point3D::new(1.0, 0.0, 0.0)),
            Colour::BLACK
        );

        assert_eq!(
            pattern.colour_at(Point3D::new(1.0, 0.0, 1.0)),
            Colour::WHITE
        );
    }

    #[test]
    fn a_gradient_pattern_linearly_interpolates_between_the_given_colours_along_x() {
        let pattern = Pattern::gradient(Colour::WHITE, Colour::BLACK);

        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 0.0, 0.0)),
            Colour::WHITE
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(0.25, 0.0, 0.0)),
            Colour::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(0.5, 0.0, 0.0)),
            Colour::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(0.75, 0.0, 0.0)),
            Colour::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn a_ring_pattern_alternates_primary_and_secondary_colour_rings() {
        let pattern = Pattern::ring(Colour::WHITE, Colour::BLACK);

        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 0.0, 0.0)),
            Colour::WHITE
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(1.0, 0.0, 0.0)),
            Colour::BLACK
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 0.0, 1.0)),
            Colour::BLACK
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(0.708, 0.0, 0.708)),
            Colour::BLACK
        );
    }

    #[test]
    fn a_checkers_pattern_alternates_in_integer_increments_of_x() {
        let pattern = Pattern::checkers(Colour::WHITE, Colour::BLACK);

        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 0.0, 0.0)),
            Colour::WHITE
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(0.99, 0.0, 0.0)),
            Colour::WHITE
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(1.0, 0.0, 0.0)),
            Colour::BLACK
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(2.0, 0.0, 0.0)),
            Colour::WHITE
        );
    }

    #[test]
    fn a_checkers_pattern_alternates_in_integer_increments_of_y() {
        let pattern = Pattern::checkers(Colour::WHITE, Colour::BLACK);

        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 0.0, 0.0)),
            Colour::WHITE
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 0.99, 0.0)),
            Colour::WHITE
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 1.0, 0.0)),
            Colour::BLACK
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 2.0, 0.0)),
            Colour::WHITE
        );
    }

    #[test]
    fn a_checkers_pattern_alternates_in_integer_increments_of_z() {
        let pattern = Pattern::checkers(Colour::WHITE, Colour::BLACK);

        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 0.0, 0.0)),
            Colour::WHITE
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 0.0, 0.99)),
            Colour::WHITE
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 0.0, 1.0)),
            Colour::BLACK
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 0.0, 2.0)),
            Colour::WHITE
        );
    }

    #[test]
    fn a_checkers_pattern_should_round_very_small_fractions_to_the_nearest_integer_rather_than_flooring(
    ) {
        let pattern = Pattern::checkers(Colour::WHITE, Colour::BLACK);

        assert_eq!(pattern.colour_at(Point3D::ORIGIN), Colour::WHITE);
        assert_eq!(
            pattern.colour_at(Point3D::new(f64::EPSILON, f64::EPSILON, -f64::EPSILON)),
            Colour::WHITE
        );

        let slightly_larger = 1.0 + f64::EPSILON;
        let slightly_smaller = 1.0 - f64::EPSILON;

        assert_eq!(
            pattern.colour_at(Point3D::new(1.0, 1.0, 1.0)),
            Colour::BLACK
        );
        assert_eq!(
            pattern.colour_at(Point3D::new(
                slightly_smaller,
                slightly_larger,
                slightly_larger
            )),
            Colour::BLACK
        );
    }

    #[test]
    fn a_checker_uv_pattern_alternates_between_the_two_colours() {
        let pattern = UvPattern {
            kind: UvPatternKind::Checkers(Colour::BLACK, Colour::WHITE),
            width: 2,
            height: 2,
        };

        vec![
            (0.0, 0.0, Colour::BLACK),
            (0.5, 0.0, Colour::WHITE),
            (0.0, 0.5, Colour::WHITE),
            (0.5, 0.5, Colour::BLACK),
            (1.0, 1.0, Colour::BLACK),
        ]
        .into_iter()
        .for_each(|(u, v, expected)| assert_eq!(pattern.colour_at((u, v)), expected))
    }

    #[test]
    fn a_checker_uv_pattern_should_handle_slight_floating_point_errors_correctly() {
        let pattern = UvPattern {
            kind: UvPatternKind::Checkers(Colour::BLACK, Colour::WHITE),
            width: 2,
            height: 2,
        };

        assert_eq!(pattern.colour_at((1.0, 1.0)), Colour::BLACK);
        assert_eq!(pattern.colour_at((1.0, 1.0 - f64::EPSILON)), Colour::BLACK);
    }

    #[test]
    fn an_alignment_check_pattern_should_have_different_colours_in_each_corner() {
        let pattern = UvPattern {
            kind: UvPatternKind::AlignmentCheck {
                main: Colour::WHITE,
                top_left: Colour::RED,
                top_right: Colour::new(1.0, 1.0, 0.0),
                bottom_left: Colour::GREEN,
                bottom_right: Colour::new(0.0, 1.0, 1.0),
            },
            width: 1,
            height: 1,
        };

        vec![
            ((0.5, 0.5), Colour::WHITE),
            ((0.1, 0.9), Colour::RED),
            ((0.9, 0.9), Colour::new(1.0, 1.0, 0.0)),
            ((0.1, 0.1), Colour::GREEN),
            ((0.9, 0.1), Colour::new(0.0, 1.0, 1.0)),
        ]
        .into_iter()
        .for_each(|((u, v), expected)| {
            assert_eq!(pattern.colour_at((u, v)), expected);
        })
    }

    #[test]
    fn an_alignment_check_pattern_should_tesselate_correctly() {
        let pattern = UvPattern {
            kind: UvPatternKind::AlignmentCheck {
                main: Colour::WHITE,
                top_left: Colour::RED,
                top_right: Colour::new(1.0, 1.0, 0.0),
                bottom_left: Colour::GREEN,
                bottom_right: Colour::new(0.0, 1.0, 1.0),
            },
            width: 1,
            height: 1,
        };

        assert_eq!(pattern.colour_at((1.1, 1.1)), Colour::GREEN);
    }

    #[rustfmt::skip]
    #[test]
    fn a_cubic_uv_should_map_uvs_to_one_of_six_faces() {
        let uv = UvPattern::cubic(
            UvPattern::alignment_check(
                Colour::new(0.0, 1.0, 1.0),
                Colour::RED,
                Colour::new(1.0, 1.0, 0.0),
                Colour::new(1.0, 0.5, 0.0),
                Colour::GREEN,
            ),
            UvPattern::alignment_check(
                Colour::GREEN,
                Colour::new(1.0, 0.0, 1.0),
                Colour::new(0.0, 1.0, 1.0),
                Colour::WHITE,
                Colour::BLUE,
            ),
            UvPattern::alignment_check(
                Colour::new(1.0, 1.0, 0.0),
                Colour::new(0.0, 1.0, 1.0),
                Colour::RED,
                Colour::BLUE,
                Colour::new(1.0, 0.5, 0.0),
            ),
            UvPattern::alignment_check(
                Colour::RED,
                Colour::new(1.0, 1.0, 0.0),
                Colour::new(1.0, 0.0, 1.0),
                Colour::GREEN,
                Colour::WHITE,
            ),
            UvPattern::alignment_check(
                Colour::new(1.0, 0.5, 0.0),
                Colour::new(0.0, 1.0, 1.0),
                Colour::new(1.0, 0.0, 1.0),
                Colour::RED,
                Colour::new(1.0, 1.0, 0.0),
            ),
            UvPattern::alignment_check(
                Colour::new(1.0, 0.0, 1.0),
                Colour::new(1.0, 0.5, 0.0),
                Colour::GREEN,
                Colour::BLUE,
                Colour::WHITE,
            ),
        );
        
        vec![
            ("left",   (1.5, 3.5), Colour::new(1.0, 1.0, 0.0)),
            ("left",   (1.05, 3.95), Colour::new(0.0, 1.0, 1.0)),
            ("left",   (1.95, 3.95), Colour::RED),
            ("left",   (1.05, 3.05), Colour::BLUE),
            ("left",   (1.95, 3.05), Colour::new(1.0, 0.5, 0.0)),
            ("front",  (0.5, 2.5), Colour::new(0.0, 1.0, 1.0)),
            ("front",  (0.05, 2.95), Colour::RED),
            ("front",  (0.95, 2.95), Colour::new(1.0, 1.0, 0.0)),
            ("front",  (0.05, 2.05), Colour::new(1.0, 0.5, 0.0)),
            ("front",  (0.95, 2.05), Colour::GREEN),
            ("right",  (1.5, 1.5), Colour::RED),
            ("right",  (1.05, 1.95), Colour::new(1.0, 1.0, 0.0)),
            ("right",  (1.95, 1.95), Colour::new(1.0, 0.0, 1.0)),
            ("right",  (1.05, 1.05), Colour::GREEN),
            ("right",  (1.95, 1.05), Colour::WHITE),
            ("back",   (2.5, 2.5), Colour::GREEN),
            ("back",   (2.05, 2.95), Colour::new(1.0, 0.0, 1.0)),
            ("back",   (2.95, 2.95), Colour::new(0.0, 1.0, 1.0)),
            ("back",   (2.05, 2.05), Colour::WHITE),
            ("back",   (2.95, 2.05), Colour::BLUE),
            ("top",    (1.5, 0.5), Colour::new(1.0, 0.5, 0.0)),
            ("top",    (1.05, 0.95), Colour::new(0.0, 1.0, 1.0)),
            ("top",    (1.95, 0.95), Colour::new(1.0, 0.0, 1.0)),
            ("top",    (1.05, 0.05), Colour::RED),
            ("top",    (1.95, 0.05), Colour::new(1.0, 1.0, 0.0)),
            ("bottom", (1.5, 2.5), Colour::new(1.0, 0.0, 1.0)),
            ("bottom", (1.05, 2.95), Colour::new(1.0, 0.5, 0.0)),
            ("bottom", (1.95, 2.95), Colour::GREEN),
            ("bottom", (1.05, 2.05), Colour::BLUE),
            ("bottom", (1.95, 2.05), Colour::WHITE),
        ]
            .into_iter()
            .for_each(|(side, (u, v), expected)| {
                assert_eq!(uv.colour_at((u, v)), expected, "{} side", side)
            })
    }
}

mod property_tests {
    use super::*;
    use crate::util::ReasonableF64;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn a_striped_pattern_is_constant_across_y_values(y: ReasonableF64) {
        let pattern = Pattern::striped(Colour::WHITE, Colour::BLACK);

        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, y.0, 0.0)),
            Colour::WHITE
        );
    }

    #[quickcheck]
    fn a_striped_pattern_is_constant_across_z_values(z: ReasonableF64) {
        let pattern = Pattern::striped(Colour::WHITE, Colour::BLACK);

        assert_eq!(
            pattern.colour_at(Point3D::new(0.0, 0.0, z.0)),
            Colour::WHITE
        );
    }
}
