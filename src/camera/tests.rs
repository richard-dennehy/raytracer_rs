use super::*;

mod unit_tests {
    use super::*;
    use crate::Vector3D;
    use std::f64::consts::{PI, SQRT_2};

    #[test]
    fn should_calculate_pixel_size_for_a_landscape_view() {
        let camera = Camera::new(
            NonZeroU16::new(200).unwrap(),
            NonZeroU16::new(125).unwrap(),
            PI / 2.0,
            Transform::identity(),
        );
        assert!(approx_eq!(f64, camera.pixel_size, 0.01));
    }

    #[test]
    fn should_calculate_pixel_size_for_a_portrait_view() {
        let camera = Camera::new(
            NonZeroU16::new(125).unwrap(),
            NonZeroU16::new(200).unwrap(),
            PI / 2.0,
            Transform::identity(),
        );
        assert!(approx_eq!(f64, camera.pixel_size, 0.01));
    }

    #[test]
    fn a_ray_through_the_centre_of_the_camera_should_travel_along_negative_z_from_the_world_origin()
    {
        let camera = Camera::new(
            NonZeroU16::new(201).unwrap(),
            NonZeroU16::new(101).unwrap(),
            PI / 2.0,
            Transform::identity(),
        );

        let ray = camera.ray_at(100, 50, 0.5, 0.5);
        assert_eq!(ray.origin, Point3D::new(0.0, 0.0, 0.0));
        assert!(approx_eq!(
            Vector3D,
            ray.direction,
            Vector3D::new(0.0, 0.0, -1.0)
        ))
    }

    #[test]
    fn a_ray_through_a_corner_of_the_canvas_should_travel_from_the_world_origin_along_the_edge_of_the_field_of_view(
    ) {
        let camera = Camera::new(
            NonZeroU16::new(201).unwrap(),
            NonZeroU16::new(101).unwrap(),
            PI / 2.0,
            Transform::identity(),
        );

        let ray = camera.ray_at(0, 0, 0.5, 0.5);
        assert_eq!(ray.origin, Point3D::new(0.0, 0.0, 0.0));
        assert!(
            approx_eq!(
                Vector3D,
                ray.direction,
                Vector3D::new(0.6651864261194508, 0.3325932130597254, -0.6685123582500481)
            ),
            "not approximately equal to {:?}",
            ray.direction
        )
    }

    #[test]
    fn a_ray_through_a_transformed_camera_should_have_the_inverse_transform_applied() {
        let transform = Transform::identity()
            .translate_y(-2.0)
            .translate_z(5.0)
            .rotate_y(PI / 4.0);

        let camera = Camera::new(
            NonZeroU16::new(201).unwrap(),
            NonZeroU16::new(101).unwrap(),
            PI / 2.0,
            transform,
        );

        let ray = camera.ray_at(100, 50, 0.5, 0.5);
        assert_eq!(ray.origin, Point3D::new(0.0, 2.0, -5.0));
        assert!(
            approx_eq!(
                Vector3D,
                ray.direction,
                Vector3D::new(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0)
            ),
            "not approximately equal to {:?}",
            ray.direction
        )
    }
}
