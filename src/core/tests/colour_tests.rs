use super::*;

mod unit_tests {
    use super::*;
    use approx::*;

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
        assert_abs_diff_eq!(difference.red(), 0.2);
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
        assert_abs_diff_eq!(product.blue(), 0.04);
    }

    #[test]
    fn two_colours_with_an_unnoticeably_different_red_should_be_similar() {
        let first = Colour::new(0.999, 0.0, 0.0);
        let second = Colour::new(0.998, 0.0, 0.0);

        assert_eq!(
            (first.red() * 255.0) as usize,
            (second.red() * 255.0) as usize
        );

        assert!(first.is_similar_to(&second));
    }

    #[test]
    fn two_colours_with_an_unnoticeably_different_green_should_be_similar() {
        let first = Colour::new(0.0, 0.499, 0.0);
        let second = Colour::new(0.0, 0.4998, 0.0);

        assert_eq!(
            (first.green() * 255.0) as usize,
            (second.green() * 255.0) as usize
        );

        assert!(first.is_similar_to(&second));
    }

    #[test]
    fn two_colours_with_an_unnoticeably_different_blue_should_be_similar() {
        let first = Colour::new(0.0, 0.0, 0.0);
        let second = Colour::new(0.0, 0.0, 0.001);

        assert_eq!(
            (first.blue() * 255.0) as usize,
            (second.blue() * 255.0) as usize
        );

        assert!(first.is_similar_to(&second));
    }

    #[test]
    fn two_colours_with_an_unnoticeably_different_red_green_and_blue_should_be_similar() {
        let first = Colour::new(0.999, 0.501, 0.0);
        let second = Colour::new(0.998, 0.499, 0.001);

        assert_eq!(
            (first.red() * 255.0) as usize,
            (second.red() * 255.0) as usize
        );
        assert_eq!(
            (first.green() * 255.0) as usize,
            (second.green() * 255.0) as usize
        );
        assert_eq!(
            (first.blue() * 255.0) as usize,
            (second.blue() * 255.0) as usize
        );

        assert!(first.is_similar_to(&second));
    }

    #[test]
    fn two_colours_with_a_small_but_noticeable_difference_should_not_be_similar() {
        let first = Colour::new(0.999, 0.502, 0.0);
        let second = Colour::new(0.996, 0.499, 0.01);

        assert_ne!(
            (first.red() * 255.0) as usize,
            (second.red() * 255.0) as usize
        );
        assert_ne!(
            (first.blue() * 255.0) as usize,
            (second.blue() * 255.0) as usize
        );
        assert_ne!(
            (first.green() * 255.0) as usize,
            (second.green() * 255.0) as usize
        );

        assert!(!first.is_similar_to(&second));
    }

    #[test]
    fn a_colour_with_a_red_of_1_and_a_colour_with_a_red_less_than_1_should_not_be_similar() {
        let first = Colour::new(1.0, 0.0, 0.0);
        let second = Colour::new(0.999999999, 0.0, 0.0);

        assert_ne!(
            (first.red() * 255.0) as usize,
            (second.red() * 255.0) as usize
        );

        assert!(!first.is_similar_to(&second));
    }
}

mod property_tests {
    use super::*;
    use crate::util::ReasonableF64;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;
    use rand::prelude::*;

    impl Arbitrary for Colour {
        fn arbitrary(_: &mut Gen) -> Self {
            let mut rng = rand::thread_rng();
            // generate a finite f64 value between 0 and 1.5
            // (colour components are allowed to be greater than 1, but allowing very large values may lead to massive rounding errors)
            fn gen_component(rng: &mut ThreadRng) -> f64 {
                rng.gen_range(0.0..1.5)
            }

            Self::new(
                gen_component(&mut rng),
                gen_component(&mut rng),
                gen_component(&mut rng),
            )
        }
    }

    #[quickcheck]
    fn adding_two_colours_should_sum_components(c1: Colour, c2: Colour) {
        let sum = c1 + c2;
        assert_eq!(sum.red(), c1.red() + c2.red());
        assert_eq!(sum.green(), c1.green() + c2.green());
        assert_eq!(sum.blue(), c1.blue() + c2.blue());
    }

    #[quickcheck]
    fn adding_two_colours_is_commutative(c1: Colour, c2: Colour) {
        assert_eq!(c1 + c2, c2 + c1);
    }

    #[quickcheck]
    fn multiplying_a_colour_by_a_scalar_should_scale_components(colour: Colour, s: ReasonableF64) {
        let s = s.0;
        let scaled = colour * s;

        assert_eq!(scaled.red(), colour.red() * s);
        assert_eq!(scaled.green(), colour.green() * s);
        assert_eq!(scaled.blue(), colour.blue() * s);
    }

    #[quickcheck]
    fn multiplying_two_colours_should_multiply_components(c1: Colour, c2: Colour) {
        let product = c1 * c2;
        assert_eq!(product.red(), c1.red() * c2.red());
        assert_eq!(product.green(), c1.green() * c2.green());
        assert_eq!(product.blue(), c1.blue() * c2.blue());
    }

    #[quickcheck]
    fn multiplying_colours_is_commutative(c1: Colour, c2: Colour) {
        assert_eq!(c1 * c2, c2 * c1);
    }
}
