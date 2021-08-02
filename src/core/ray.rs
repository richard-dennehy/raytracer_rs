use crate::core::Point3D;
use crate::core::{Matrix4D, Normal3D, Vector3D};

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3D,
    // a ray should be normalised when created, but may be stretched or squashed
    pub direction: Vector3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Normal3D) -> Self {
        Ray {
            origin,
            direction: direction.into(),
        }
    }

    pub fn position(&self, time: f64) -> Point3D {
        self.origin + self.direction * time
    }

    pub fn transformed(&self, transformation: &Matrix4D) -> Self {
        let (x, y, z, _) = transformation * self.origin;
        let origin = Point3D::new(x, y, z);

        let (x, y, z, _) = transformation * self.direction;
        let direction = Vector3D::new(x, y, z);

        Ray { origin, direction }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Transform;

    #[test]
    fn should_be_able_to_calculate_the_position_of_a_ray_at_a_given_time() {
        let ray = Ray::new(Point3D::new(2.0, 3.0, 4.0), Normal3D::POSITIVE_X);

        assert_eq!(ray.position(0.0), Point3D::new(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Point3D::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Point3D::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Point3D::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn a_ray_can_be_translated() {
        let matrix = Transform::identity()
            .translate_x(3.0)
            .translate_y(4.0)
            .translate_z(5.0);
        let ray = Ray::new(Point3D::new(1.0, 2.0, 3.0), Normal3D::POSITIVE_Y);

        let transformed = ray.transformed(&matrix.underlying());
        assert_eq!(transformed.origin, Point3D::new(4.0, 6.0, 8.0));
        assert_eq!(transformed.direction, Vector3D::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn a_ray_can_be_scaled() {
        let matrix = Transform::identity().scale_x(2.0).scale_y(3.0).scale_z(4.0);
        let ray = Ray::new(Point3D::new(1.0, 2.0, 3.0), Normal3D::POSITIVE_Y);

        let transformed = ray.transformed(&matrix.underlying());
        assert_eq!(transformed.origin, Point3D::new(2.0, 6.0, 12.0));
        assert_eq!(transformed.direction, Vector3D::new(0.0, 3.0, 0.0));
    }
}
