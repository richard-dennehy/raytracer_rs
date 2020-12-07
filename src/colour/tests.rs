use super::*;

mod unit_tests {
    use super::*;

    #[test]
    fn the_first_component_is_red() {
        assert_eq!(Colour::new(-0.5, 0.4, 1.7).red(), -0.5);
    }

    #[test]
    fn the_second_component_is_green() {
        assert_eq!(Colour::new(-0.5, 0.4, 1.7).green(), 0.4);
    }

    #[test]
    fn the_third_component_is_blue() {
        assert_eq!(Colour::new(-0.5, 0.4, 1.7).blue(), 1.7);
    }

    #[test]
    fn adding_two_colours_should_sum_components() {
        let c1 = Colour::new(0.9, 0.6, 0.75);
        let c2 = Colour::new(0.7, 0.1, 0.25);

        let sum = c1 + c2;
        assert_eq!(sum.red(), 1.6);
        assert_eq!(sum.green(), 0.7);
        assert_eq!(sum.blue(), 1.0);
    }

    #[test]
    fn subtracting_a_colour_from_a_colour_should_subtract_components() {
        let c1 = Colour::new(0.9, 0.6, 0.75);
        let c2 = Colour::new(0.7, 0.1, 0.25);

        let difference = c1 - c2;
        assert!(approx_eq!(f64, difference.red(), 0.2));
        assert_eq!(difference.green(), 0.5);
        assert_eq!(difference.blue(), 0.5);
    }

    #[test]
    fn multiplying_a_colour_by_a_scalar_should_scale_components() {
        let colour = Colour::new(0.2, 0.3, 0.4);
        let scaled = colour * 2.0;

        assert_eq!(scaled.red(), 0.4);
        assert_eq!(scaled.green(), 0.6);
        assert_eq!(scaled.blue(), 0.8);
    }

    #[test]
    fn multiplying_two_colours_should_multiply_components() {
        let c1 = Colour::new(1.0, 0.2, 0.4);
        let c2 = Colour::new(0.9, 1.0, 0.1);

        let product = c1 * c2;
        assert_eq!(product.red(), 0.9);
        assert_eq!(product.green(), 0.2);
        assert!(approx_eq!(f64, product.blue(), 0.04));
    }
}

mod property_tests {
    extern crate quickcheck;

    use self::quickcheck::{Arbitrary, Gen};
    use super::*;

    impl Arbitrary for Colour {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Colour::new(f64::arbitrary(g), f64::arbitrary(g), f64::arbitrary(g))
        }
    }

    #[quickcheck]
    fn a_colour_has_red_blue_and_green_components(r: f64, g: f64, b: f64) {
        let colour = Colour::new(r, g, b);

        assert_eq!(colour.red(), r);
        assert_eq!(colour.green(), g);
        assert_eq!(colour.blue(), b);
    }

    #[quickcheck]
    fn adding_two_colours_should_sum_components(
        r1: f64,
        g1: f64,
        b1: f64,
        r2: f64,
        g2: f64,
        b2: f64,
    ) {
        let c1 = Colour::new(r1, g1, b1);
        let c2 = Colour::new(r2, g2, b2);

        let sum = c1 + c2;
        assert_eq!(sum.red(), r1 + r2);
        assert_eq!(sum.green(), g1 + g2);
        assert_eq!(sum.blue(), b1 + b2);
    }

    #[quickcheck]
    fn adding_two_colours_is_commutative(c1: Colour, c2: Colour) {
        assert_eq!(c1 + c2, c2 + c1);
    }

    #[quickcheck]
    fn multiplying_a_colour_by_a_scalar_should_scale_components(r: f64, g: f64, b: f64, s: f64) {
        let colour = Colour::new(r, g, b);
        let scaled = colour * s;

        assert_eq!(scaled.red(), r * s);
        assert_eq!(scaled.green(), g * s);
        assert_eq!(scaled.blue(), b * s);
    }

    #[quickcheck]
    fn multiplying_two_colours_should_multiply_components(
        r1: f64,
        g1: f64,
        b1: f64,
        r2: f64,
        g2: f64,
        b2: f64,
    ) {
        let c1 = Colour::new(r1, g1, b1);
        let c2 = Colour::new(r2, g2, b2);

        let sum = c1 * c2;
        assert_eq!(sum.red(), r1 * r2);
        assert_eq!(sum.green(), g1 * g2);
        assert_eq!(sum.blue(), b1 * b2);
    }

    #[quickcheck]
    fn multiplying_colours_is_commutative(c1: Colour, c2: Colour) {
        assert_eq!(c1 * c2, c2 * c1);
    }
}
