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

        let canvas = render(World::default(), camera, Subsamples::None);
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
    fn a_sample_grid_of_1_should_cast_a_single_ray_at_the_centre() {
        let samples = Samples::new(1);
        let mut offsets = samples.offsets();
        assert_eq!(offsets.next().unwrap(), &(0.5, 0.5));
        assert_eq!(offsets.next(), None);
    }

    #[test]
    fn a_sample_grid_of_2_should_cast_4_rays() {
        let samples = Samples::new(2);
        let mut offsets = samples.offsets();
        assert_eq!(offsets.next().unwrap(), &(0.25, 0.25));
        assert_eq!(offsets.next().unwrap(), &(0.75, 0.25));
        assert_eq!(offsets.next().unwrap(), &(0.25, 0.75));
        assert_eq!(offsets.next().unwrap(), &(0.75, 0.75));
        assert_eq!(offsets.next(), None);
    }

    #[test]
    fn a_sample_grid_of_4_should_cast_16_rays() {
        let samples = Samples::new(4);
        let mut offsets = samples.offsets();

        assert_eq!(offsets.next().unwrap(), &(0.125, 0.125));
        assert_eq!(offsets.next().unwrap(), &(0.375, 0.125));
        assert_eq!(offsets.next().unwrap(), &(0.625, 0.125));
        assert_eq!(offsets.next().unwrap(), &(0.875, 0.125));

        assert_eq!(offsets.next().unwrap(), &(0.125, 0.375));
        assert_eq!(offsets.next().unwrap(), &(0.375, 0.375));
        assert_eq!(offsets.next().unwrap(), &(0.625, 0.375));
        assert_eq!(offsets.next().unwrap(), &(0.875, 0.375));

        assert_eq!(offsets.next().unwrap(), &(0.125, 0.625));
        assert_eq!(offsets.next().unwrap(), &(0.375, 0.625));
        assert_eq!(offsets.next().unwrap(), &(0.625, 0.625));
        assert_eq!(offsets.next().unwrap(), &(0.875, 0.625));

        assert_eq!(offsets.next().unwrap(), &(0.125, 0.875));
        assert_eq!(offsets.next().unwrap(), &(0.375, 0.875));
        assert_eq!(offsets.next().unwrap(), &(0.625, 0.875));
        assert_eq!(offsets.next().unwrap(), &(0.875, 0.875));

        assert_eq!(offsets.next(), None);
    }
}
