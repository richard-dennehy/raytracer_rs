use super::*;

mod unit_tests {
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

        let canvas = render(World::default(), camera);
        assert!(approx_eq!(
            Colour,
            canvas.get(5, 5),
            Colour::new(
                0.38066119308103435,
                0.47582649135129296,
                0.28549589481077575
            ),
            epsilon = f32::EPSILON as f64
        ));
    }
}
