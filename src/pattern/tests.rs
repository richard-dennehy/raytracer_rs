use super::*;

mod unit_tests {
    use super::*;
    use std::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

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
    fn a_spherical_uv_map_should_convert_3d_points_to_2d_uv() {
        vec![
            (Point3D::new(0.0, 0.0, -1.0), (0.0, 0.5)),
            (Point3D::new(1.0, 0.0, 0.0), (0.25, 0.5)),
            (Point3D::new(0.0, 0.0, 1.0), (0.5, 0.5)),
            (Point3D::new(-1.0, 0.0, 0.0), (0.75, 0.5)),
            (Point3D::new(0.0, 1.0, 0.0), (0.5, 1.0)),
            (Point3D::new(0.0, -1.0, 0.0), (0.5, 0.0)),
            (Point3D::new(SQRT_2 / 2.0, SQRT_2 / 2.0, 0.0), (0.25, 0.75)),
        ]
        .into_iter()
        .for_each(|(point, (u, v))| {
            assert_eq!(UvMap::Spherical.uv(point), (u, v));
        })
    }

    #[test]
    fn a_uv_checker_pattern_and_spherical_map_should_alternate_colours_across_a_sphere() {
        let pattern = Pattern::texture(
            UvPattern::checkers(
                Colour::BLACK,
                Colour::WHITE,
                nonzero_ext::nonzero!(16_usize),
                nonzero_ext::nonzero!(8_usize),
            ),
            UvMap::Spherical,
        );

        vec![
            (Point3D::new(0.4315, 0.4670, 0.7719), Colour::WHITE),
            (Point3D::new(-0.9654, 0.2552, -0.0534), Colour::BLACK),
            (Point3D::new(0.1039, 0.7090, 0.6975), Colour::WHITE),
            (Point3D::new(-0.4986, -0.7856, -0.3663), Colour::BLACK),
            (Point3D::new(-0.0317, -0.9395, 0.3411), Colour::BLACK),
            (Point3D::new(0.4809, -0.7721, 0.4154), Colour::BLACK),
            (Point3D::new(0.0285, -0.9612, -0.2745), Colour::BLACK),
            (Point3D::new(-0.5734, -0.2162, -0.7903), Colour::WHITE),
            (Point3D::new(0.7688, -0.1470, 0.6223), Colour::BLACK),
            (Point3D::new(-0.7652, 0.2175, 0.6060), Colour::BLACK),
        ]
        .into_iter()
        .for_each(|(point, expected)| assert_eq!(pattern.colour_at(point), expected))
    }

    #[test]
    fn a_planar_map_should_project_3d_points_onto_a_2d_plane() {
        vec![
            (Point3D::new(0.25, 0.0, 0.5), (0.25, 0.5)),
            (Point3D::new(0.25, 0.0, -0.25), (0.25, 0.75)),
            (Point3D::new(0.25, 0.5, -0.25), (0.25, 0.75)),
            (Point3D::new(1.25, 0.0, 0.5), (0.25, 0.5)),
            (Point3D::new(0.25, 0.0, -1.75), (0.25, 0.25)),
            (Point3D::new(1.0, 0.0, -1.0), (0.0, 0.0)),
            (Point3D::ORIGIN, (0.0, 0.0)),
        ]
        .into_iter()
        .for_each(|(point, (u, v))| {
            assert_eq!(UvMap::Planar.uv(point), (u, v));
        })
    }

    #[test]
    fn a_cylindrical_map_should_project_3d_points_onto_an_unwrapped_cylinder() {
        vec![
            (Point3D::new(0.0, 0.0, -1.0), (0.0, 0.0)),
            (Point3D::new(0.0, 0.5, -1.0), (0.0, 0.5)),
            (Point3D::new(0.0, 1.0, -1.0), (0.0, 0.0)),
            (
                Point3D::new(FRAC_1_SQRT_2, 0.5, -FRAC_1_SQRT_2),
                (0.125, 0.5),
            ),
            (Point3D::new(1.0, 0.5, 0.0), (0.25, 0.5)),
            (
                Point3D::new(FRAC_1_SQRT_2, 0.5, FRAC_1_SQRT_2),
                (0.375, 0.5),
            ),
            (Point3D::new(0.0, -0.25, 1.0), (0.5, 0.75)),
            (
                Point3D::new(-FRAC_1_SQRT_2, 0.5, FRAC_1_SQRT_2),
                (0.625, 0.5),
            ),
            (Point3D::new(-1.0, 1.25, 0.0), (0.75, 0.25)),
            (
                Point3D::new(-FRAC_1_SQRT_2, 0.5, -FRAC_1_SQRT_2),
                (0.875, 0.5),
            ),
        ]
        .into_iter()
        .for_each(|(point, (u, v))| {
            assert_eq!(UvMap::Cylindrical.uv(point), (u, v));
        })
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
    fn a_cubic_texture_should_map_cube_points_onto_one_of_6_faces() {
        let pattern = Pattern::cubic_texture(
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
            ("left", Point3D::new(-1.0, 0.0, 0.0), Colour::new(1.0, 1.0, 0.0)),
            ("left", Point3D::new(-1.0, 0.9, -0.9), Colour::new(0.0, 1.0, 1.0)),
            ("left", Point3D::new(-1.0, 0.9, 0.9), Colour::RED),
            ("left", Point3D::new(-1.0, -0.9, -0.9), Colour::BLUE),
            ("left", Point3D::new(-1.0, -0.9, 0.9), Colour::new(1.0, 0.5, 0.0)),
            ("front", Point3D::new(0.0, 0.0, 1.0), Colour::new(0.0, 1.0, 1.0)),
            ("front", Point3D::new(-0.9, 0.9, 1.0), Colour::RED),
            ("front", Point3D::new(0.9, 0.9, 1.0), Colour::new(1.0, 1.0, 0.0)),
            ("front", Point3D::new(-0.9, -0.9, 1.0), Colour::new(1.0, 0.5, 0.0)),
            ("front", Point3D::new(0.9, -0.9, 1.0), Colour::GREEN),
            ("right", Point3D::new(1.0, 0.0, 0.0), Colour::RED),
            ("right", Point3D::new(1.0, 0.9, 0.9), Colour::new(1.0, 1.0, 0.0)),
            ("right", Point3D::new(1.0, 0.9, -0.9), Colour::new(1.0, 0.0, 1.0)),
            ("right", Point3D::new(1.0, -0.9, 0.9), Colour::GREEN),
            ("right", Point3D::new(1.0, -0.9, -0.9), Colour::WHITE),
            ("back", Point3D::new(0.0, 0.0, -1.0), Colour::GREEN),
            ("back", Point3D::new(0.9, 0.9, -1.0), Colour::new(1.0, 0.0, 1.0)),
            ("back", Point3D::new(-0.9, 0.9, -1.0), Colour::new(0.0, 1.0, 1.0)),
            ("back", Point3D::new(0.9, -0.9, -1.0), Colour::WHITE),
            ("back", Point3D::new(-0.9, -0.9, -1.0), Colour::BLUE),
            ("top", Point3D::new(0.0, 1.0, 0.0), Colour::new(1.0, 0.5, 0.0)),
            ("top", Point3D::new(-0.9, 1.0, -0.9), Colour::new(0.0, 1.0, 1.0)),
            ("top", Point3D::new(0.9, 1.0, -0.9), Colour::new(1.0, 0.0, 1.0)),
            ("top", Point3D::new(-0.9, 1.0, 0.9), Colour::RED),
            ("top", Point3D::new(0.9, 1.0, 0.9), Colour::new(1.0, 1.0, 0.0)),
            ("bottom", Point3D::new(0.0, -1.0, 0.0), Colour::new(1.0, 0.0, 1.0)),
            ("bottom", Point3D::new(-0.9, -1.0, 0.9), Colour::new(1.0, 0.5, 0.0)),
            ("bottom", Point3D::new(0.9, -1.0, 0.9), Colour::GREEN),
            ("bottom", Point3D::new(-0.9, -1.0, -0.9), Colour::BLUE),
            ("bottom", Point3D::new(0.9, -1.0, -0.9), Colour::WHITE),
        ]
        .into_iter()
        .for_each(|(side, point, expected)| {
            assert_eq!(pattern.colour_at(point), expected, "{} side", side)
        })
    }
}

mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn a_solid_pattern_should_have_the_same_colour_at_all_points(
            point in any::<Point3D>(),
            red in crate::util::reasonable_f64(),
            green in crate::util::reasonable_f64(),
            blue in crate::util::reasonable_f64(),
        ) {
            let colour = Colour::new(red, green, blue);
            let pattern = Pattern::solid(colour);

            assert_eq!(pattern.colour_at(point), colour);
        }

        #[test]
        fn a_striped_pattern_is_constant_across_y_values(y in crate::util::reasonable_f64()) {
            let pattern = Pattern::striped(Colour::WHITE, Colour::BLACK);

            assert_eq!(pattern.colour_at(Point3D::new(0.0, y, 0.0)), Colour::WHITE);
        }

        #[test]
        fn a_striped_pattern_is_constant_across_z_values(z in crate::util::reasonable_f64()) {
            let pattern = Pattern::striped(Colour::WHITE, Colour::BLACK);

            assert_eq!(pattern.colour_at(Point3D::new(0.0, 0.0, z)), Colour::WHITE);
        }
    }
}
