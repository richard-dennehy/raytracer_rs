use super::*;

mod unit_tests {
    use super::*;

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
