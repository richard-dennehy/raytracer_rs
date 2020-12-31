use super::*;

mod unit_tests {
    use super::*;
    use crate::{Colour, Matrix4D, Point3D, Vector3D};
    use std::f64::consts::PI;
    use std::num::NonZeroU16;

    #[test]
    fn rendering_with_the_default_world_should_produce_the_correct_colour_at_the_centre() {
        let view_transform = Matrix4D::view_transform(
            Point3D::new(0.0, 0.0, -5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );
        let camera = Camera::new(
            NonZeroU16::new(11).unwrap(),
            NonZeroU16::new(11).unwrap(),
            PI / 2.0,
            view_transform,
        );

        let canvas = render(World::default(), camera);
        assert_eq!(
            canvas.get(5, 5),
            Colour::new(
                0.38066119308103435,
                0.47582649135129296,
                0.28549589481077575
            )
        );
    }
}
