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
}

mod property_tests {
    use super::*;

    #[quickcheck]
    fn a_solid_pattern_should_have_the_same_colour_at_all_points(
        point: Point3D,
        red: f64,
        green: f64,
        blue: f64,
    ) {
        let colour = Colour::new(red, green, blue);
        let pattern = Pattern::solid(colour);

        assert_eq!(pattern.colour_at(point), colour);
    }

    #[quickcheck]
    fn a_striped_pattern_is_constant_across_y_values(y: f64) {
        let pattern = Pattern::striped(Colour::WHITE, Colour::BLACK);

        assert_eq!(pattern.colour_at(Point3D::new(0.0, y, 0.0)), Colour::WHITE);
    }

    #[quickcheck]
    fn a_striped_pattern_is_constant_across_z_values(z: f64) {
        let pattern = Pattern::striped(Colour::WHITE, Colour::BLACK);

        assert_eq!(pattern.colour_at(Point3D::new(0.0, 0.0, z)), Colour::WHITE);
    }
}
