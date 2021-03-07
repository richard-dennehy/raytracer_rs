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
    use super::*;

    use proptest::prelude::*;

    impl Arbitrary for Colour {
        type Parameters = ();

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            (any::<f64>(), any::<f64>(), any::<f64>())
                .prop_map(|(x, y, z)| Colour::new(x, y, z))
                .boxed()
        }

        type Strategy = BoxedStrategy<Self>;
    }

    proptest! {
        #[test]
        fn a_colour_has_red_blue_and_green_components(r in any::<f64>(), g in any::<f64>(), b in any::<f64>()) {
            let colour = Colour::new(r, g, b);

            assert_eq!(colour.red(), r);
            assert_eq!(colour.green(), g);
            assert_eq!(colour.blue(), b);
        }

        #[test]
        fn adding_two_colours_should_sum_components(
            r1 in any::<f64>(),
            g1 in any::<f64>(),
            b1 in any::<f64>(),
            r2 in any::<f64>(),
            g2 in any::<f64>(),
            b2 in any::<f64>(),
        ) {
            let c1 = Colour::new(r1, g1, b1);
            let c2 = Colour::new(r2, g2, b2);

            let sum = c1 + c2;
            assert_eq!(sum.red(), r1 + r2);
            assert_eq!(sum.green(), g1 + g2);
            assert_eq!(sum.blue(), b1 + b2);
        }

        #[test]
        fn adding_two_colours_is_commutative(c1 in any::<Colour>(), c2 in any::<Colour>()) {
            assert_eq!(c1 + c2, c2 + c1);
        }

        #[test]
        fn multiplying_a_colour_by_a_scalar_should_scale_components(r in any::<f64>(), g in any::<f64>(), b in any::<f64>(), s in any::<f64>()) {
            let colour = Colour::new(r, g, b);
            let scaled = colour * s;

            assert_eq!(scaled.red(), r * s);
            assert_eq!(scaled.green(), g * s);
            assert_eq!(scaled.blue(), b * s);
        }

        #[test]
        fn multiplying_two_colours_should_multiply_components(
            r1 in any::<f64>(),
            g1 in any::<f64>(),
            b1 in any::<f64>(),
            r2 in any::<f64>(),
            g2 in any::<f64>(),
            b2 in any::<f64>(),
        ) {
            let c1 = Colour::new(r1, g1, b1);
            let c2 = Colour::new(r2, g2, b2);

            let sum = c1 * c2;
            assert_eq!(sum.red(), r1 * r2);
            assert_eq!(sum.green(), g1 * g2);
            assert_eq!(sum.blue(), b1 * b2);
        }

        #[test]
        fn multiplying_colours_is_commutative(c1 in any::<Colour>(), c2 in any::<Colour>()) {
            assert_eq!(c1 * c2, c2 * c1);
        }
    }
}
