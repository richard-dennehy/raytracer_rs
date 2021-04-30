use super::*;

mod rendering {
    use super::*;
    use crate::{Colour, Normal3D, Point3D, Transform};
    use std::f64::consts::PI;
    use std::num::NonZeroU16;

    #[test]
    fn rendering_with_the_default_world_should_produce_the_correct_colour_at_the_centre() {
        let view_transform = Transform::view_transform(
            Point3D::new(0.0, 0.0, -5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Normal3D::POSITIVE_Y,
        );
        let camera = Camera::new(
            NonZeroU16::new(11).unwrap(),
            NonZeroU16::new(11).unwrap(),
            PI / 2.0,
            view_transform,
        );

        let canvas = render(World::default(), camera, &Samples::single());
        let expected = Colour::new(
            0.38066119308103435,
            0.47582649135129296,
            0.28549589481077575,
        );
        let actual = canvas.get(5, 5);

        assert!(
            approx_eq!(Colour, expected, actual, epsilon = f32::EPSILON as f64),
            "{:?} != {:?}",
            expected,
            actual
        );
    }
}

mod samples {
    use super::*;

    #[test]
    fn a_sample_grid_of_1_should_not_have_any_inner_rays() {
        let samples = Samples::grid(nonzero_ext::nonzero!(1u8));
        let mut offsets = samples.inner_offsets();

        assert_eq!(offsets.next(), None);
    }

    #[test]
    fn a_sample_grid_of_2_should_not_have_any_inner_rays() {
        let samples = Samples::grid(nonzero_ext::nonzero!(2u8));
        let mut offsets = samples.inner_offsets();

        assert_eq!(offsets.next(), None);
    }

    #[test]
    fn a_sample_grid_of_4_should_cast_12_inner_rays() {
        let samples = Samples::grid(nonzero_ext::nonzero!(4u8));
        let mut offsets = samples.inner_offsets();

        assert_eq!(offsets.next().unwrap(), &(0.375, 0.125));
        assert_eq!(offsets.next().unwrap(), &(0.625, 0.125));

        assert_eq!(offsets.next().unwrap(), &(0.125, 0.375));
        assert_eq!(offsets.next().unwrap(), &(0.375, 0.375));
        assert_eq!(offsets.next().unwrap(), &(0.625, 0.375));
        assert_eq!(offsets.next().unwrap(), &(0.875, 0.375));

        assert_eq!(offsets.next().unwrap(), &(0.125, 0.625));
        assert_eq!(offsets.next().unwrap(), &(0.375, 0.625));
        assert_eq!(offsets.next().unwrap(), &(0.625, 0.625));
        assert_eq!(offsets.next().unwrap(), &(0.875, 0.625));

        assert_eq!(offsets.next().unwrap(), &(0.375, 0.875));
        assert_eq!(offsets.next().unwrap(), &(0.625, 0.875));

        assert_eq!(offsets.next(), None);
    }

    #[test]
    fn a_sample_grid_of_1_should_have_one_corner_at_the_centre() {
        let samples = Samples::grid(nonzero_ext::nonzero!(1u8));
        let mut corners = samples.corner_offsets();
        assert_eq!(corners.next().unwrap(), &(0.5, 0.5));
        assert_eq!(corners.next(), None);
    }

    #[test]
    fn a_sample_grid_of_2_should_have_four_corners() {
        let samples = Samples::grid(nonzero_ext::nonzero!(2u8));
        let mut corners = samples.corner_offsets();

        assert_eq!(corners.next().unwrap(), &(0.25, 0.25));
        assert_eq!(corners.next().unwrap(), &(0.75, 0.25));
        assert_eq!(corners.next().unwrap(), &(0.25, 0.75));
        assert_eq!(corners.next().unwrap(), &(0.75, 0.75));

        assert_eq!(corners.next(), None);
    }

    #[test]
    fn a_sample_grid_of_4_should_have_four_corners() {
        let samples = Samples::grid(nonzero_ext::nonzero!(4u8));
        let mut corners = samples.corner_offsets();

        assert_eq!(corners.next().unwrap(), &(0.125, 0.125));
        assert_eq!(corners.next().unwrap(), &(0.875, 0.125));
        assert_eq!(corners.next().unwrap(), &(0.125, 0.875));
        assert_eq!(corners.next().unwrap(), &(0.875, 0.875));

        assert_eq!(corners.next(), None);
    }
}
